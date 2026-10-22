//! Authentication API: token generation and logout.

use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de, types::MessageResponse};

/// Body of `POST auth/login`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response of `POST auth/login`.
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub first_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub last_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub email: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Authentication endpoints. Obtain with [`Shiprocket::auth`].
pub struct Auth<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn auth(&self) -> Auth<'_> {
        Auth { client: self }
    }
}

impl Auth<'_> {
    /// `POST auth/login` with the configured credentials. The token is cached on the client.
    ///
    /// You rarely need to call this yourself; the client logs in on demand.
    pub async fn login(&self) -> Result<LoginResponse, Error> {
        let creds = self.client.credentials().ok_or(Error::MissingCredentials)?;
        let res: LoginResponse = self
            .client
            .send_unauthenticated(
                self.client
                    .http()
                    .post(self.client.api_url("auth/login"))
                    .json(creds),
            )
            .await?;
        self.client.set_token(res.token.clone()).await;
        Ok(res)
    }

    /// `POST auth/logout`. Invalidates the current token and clears it from the client.
    pub async fn logout(&self) -> Result<MessageResponse, Error> {
        let res = self
            .client
            .post_empty(self.client.api_url("auth/logout"))
            .await;
        self.client.clear_token().await;
        res
    }
}
