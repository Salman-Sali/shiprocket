//! Types shared across endpoint groups.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::de;

/// Generic `{ "message": ..., "status_code": ... }` style response.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MessageResponse {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// `{ "data": [...], "meta": { "pagination": ... } }` list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct Paginated<T> {
    #[serde(default, deserialize_with = "de::vec")]
    pub data: Vec<T>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub meta: Option<Meta>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// `{ "data": {...} }` single object response.
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct DataResponse<T> {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<T>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Meta {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub pagination: Option<Pagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Pagination {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub per_page: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub current_page: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_pages: Option<i64>,
    #[serde(default)]
    pub links: serde_json::Value,
}

/// PHP `DateTime` object as serialized by Shiprocket
/// (`{"date": "...", "timezone_type": 3, "timezone": "Asia/Kolkata"}`).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PhpDateTime {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub timezone_type: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub timezone: Option<String>,
}

/// Response of the bulk CSV import endpoints (orders, products, listings).
///
/// Pass `id` to [`crate::imports::Imports::check`] to see how the import went.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImportResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Payment mode of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Prepaid,
    #[serde(rename = "COD", alias = "cod", alias = "Cod")]
    Cod,
}

/// A file to upload (CSV imports, KYC documents, ...).
#[derive(Debug, Clone)]
pub struct Upload {
    pub file_name: String,
    pub bytes: Vec<u8>,
    /// Mime type, e.g. `text/csv`. Left to the server to sniff when `None`.
    pub mime: Option<String>,
}

impl Upload {
    pub fn new(file_name: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            file_name: file_name.into(),
            bytes: bytes.into(),
            mime: None,
        }
    }

    /// CSV upload with the `text/csv` mime type set.
    pub fn csv(file_name: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(file_name, bytes).mime("text/csv")
    }

    pub fn mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }

    /// Reads a file from disk (blocking).
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "upload".into());
        let mut upload = Self::new(file_name, bytes);
        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("csv"))
        {
            upload.mime = Some("text/csv".into());
        }
        Ok(upload)
    }
}
