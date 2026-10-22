use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// Errors returned by the Shiprocket client.
#[derive(Error, Debug)]
pub enum Error {
    /// Transport level failure (connection, TLS, timeout, ...).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Shiprocket answered with a non-success status code.
    #[error("shiprocket api error ({status}): {message}")]
    Api {
        status: StatusCode,
        message: String,
        /// Field level validation errors, present on most `422` responses.
        errors: HashMap<String, Vec<String>>,
        /// Raw response body.
        body: String,
    },

    /// The response body could not be decoded into the expected type.
    #[error("failed to decode response: {source}")]
    Decode {
        #[source]
        source: serde_json::Error,
        /// Raw response body.
        body: String,
    },

    /// A request needed a token but the client has neither a token nor credentials.
    #[error("no token or credentials configured")]
    MissingCredentials,

    /// Reading a file for upload failed.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    /// HTTP status code of an [`Error::Api`] error.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::Http(e) => e.status(),
            _ => None,
        }
    }

    pub(crate) fn from_response(status: StatusCode, body: String) -> Self {
        #[derive(Deserialize)]
        struct ErrorBody {
            #[serde(default)]
            message: Option<serde_json::Value>,
            #[serde(default)]
            errors: Option<serde_json::Value>,
        }

        let parsed = serde_json::from_str::<ErrorBody>(&body).ok();
        let message = parsed
            .as_ref()
            .and_then(|b| b.message.as_ref())
            .map(|m| match m {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .unwrap_or_else(|| {
                status
                    .canonical_reason()
                    .unwrap_or("unknown error")
                    .to_string()
            });
        let errors = parsed
            .and_then(|b| b.errors)
            .map(collect_field_errors)
            .unwrap_or_default();

        Self::Api {
            status,
            message,
            errors,
            body,
        }
    }
}

/// Shiprocket sends field errors as `{"field": ["msg"]}` but sometimes as `{"field": "msg"}`.
fn collect_field_errors(value: serde_json::Value) -> HashMap<String, Vec<String>> {
    let serde_json::Value::Object(map) = value else {
        return HashMap::new();
    };
    map.into_iter()
        .map(|(k, v)| {
            let msgs = match v {
                serde_json::Value::Array(items) => items
                    .into_iter()
                    .map(|i| match i {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    })
                    .collect(),
                serde_json::Value::String(s) => vec![s],
                other => vec![other.to_string()],
            };
            (k, msgs)
        })
        .collect()
}
