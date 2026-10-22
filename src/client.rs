use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::{RequestBuilder, StatusCode, multipart};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{Mutex, RwLock};

use crate::{auth::LoginRequest, error::Error, types::Upload};

/// Base url of the main Shiprocket API.
pub const DEFAULT_BASE_URL: &str = "https://apiv2.shiprocket.in";
/// Base url of the serviceability API (blocked pincodes).
pub const DEFAULT_SERVICEABILITY_BASE_URL: &str = "https://serviceability.shiprocket.in";

/// Shiprocket tokens are valid for 240 hours. Refresh a day early.
const TOKEN_TTL: Duration = Duration::from_secs(9 * 24 * 60 * 60);

/// Async Shiprocket API client.
///
/// Cheap to clone; clones share the HTTP connection pool and the cached token.
///
/// With credentials the client logs in lazily on the first request, caches the token,
/// refreshes it before it expires and re-logs in once when a request comes back `401`.
///
/// ```no_run
/// # async fn run() -> Result<(), shiprocket::Error> {
/// let client = shiprocket::Shiprocket::new("api-user@example.com", "password");
/// let balance = client.account().wallet_balance().await?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct Shiprocket {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    http: reqwest::Client,
    base_url: String,
    serviceability_base_url: String,
    credentials: Option<LoginRequest>,
    token: RwLock<Option<CachedToken>>,
    login_lock: Mutex<()>,
}

#[derive(Debug, Clone)]
struct CachedToken {
    value: String,
    /// `None` when the token was supplied by the caller and its age is unknown.
    obtained_at: Option<Instant>,
}

impl CachedToken {
    fn is_fresh(&self) -> bool {
        self.obtained_at.is_none_or(|t| t.elapsed() < TOKEN_TTL)
    }
}

impl Shiprocket {
    /// Client that logs in with the given API user credentials when needed.
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self::builder().credentials(email, password).build()
    }

    /// Client that uses an existing token. It cannot re-login when the token expires.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::builder().token(token).build()
    }

    pub fn builder() -> ShiprocketBuilder {
        ShiprocketBuilder::default()
    }

    /// The currently cached token, if any. Useful for persisting it between runs.
    pub async fn token(&self) -> Option<String> {
        self.inner
            .token
            .read()
            .await
            .as_ref()
            .map(|t| t.value.clone())
    }

    /// Replace the cached token.
    pub async fn set_token(&self, token: impl Into<String>) {
        *self.inner.token.write().await = Some(CachedToken {
            value: token.into(),
            obtained_at: Some(Instant::now()),
        });
    }

    /// Drop the cached token. The next request logs in again.
    pub async fn clear_token(&self) {
        *self.inner.token.write().await = None;
    }

    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub(crate) fn credentials(&self) -> Option<&LoginRequest> {
        self.inner.credentials.as_ref()
    }

    pub(crate) fn api_url(&self, path: &str) -> String {
        format!("{}/v1/external/{}", self.inner.base_url, path)
    }

    pub(crate) fn serviceability_url(&self, path: &str) -> String {
        format!(
            "{}/v1/external/{}",
            self.inner.serviceability_base_url, path
        )
    }

    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.inner.http
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T, Error> {
        self.send(|http| http.get(&url)).await
    }

    pub(crate) async fn get_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        url: String,
        query: &Q,
    ) -> Result<T, Error> {
        self.send(|http| http.get(&url).query(query)).await
    }

    pub(crate) async fn post<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        url: String,
        body: &B,
    ) -> Result<T, Error> {
        self.send(|http| http.post(&url).json(body)).await
    }

    pub(crate) async fn post_empty<T: DeserializeOwned>(&self, url: String) -> Result<T, Error> {
        self.send(|http| http.post(&url)).await
    }

    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        url: String,
        body: &B,
    ) -> Result<T, Error> {
        self.send(|http| http.patch(&url).json(body)).await
    }

    pub(crate) async fn put<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        url: String,
        body: &B,
    ) -> Result<T, Error> {
        self.send(|http| http.put(&url).json(body)).await
    }

    /// Multipart upload of `file` under form field `field`, plus extra text fields.
    pub(crate) async fn post_file<T: DeserializeOwned>(
        &self,
        url: String,
        field: &str,
        file: &Upload,
        fields: &[(&str, String)],
    ) -> Result<T, Error> {
        self.send(|http| {
            let mut part =
                multipart::Part::bytes(file.bytes.clone()).file_name(file.file_name.clone());
            if let Some(mime) = &file.mime {
                part = part.mime_str(mime).expect("invalid mime type");
            }
            let mut form = multipart::Form::new().part(field.to_string(), part);
            for (k, v) in fields {
                form = form.text(k.to_string(), v.clone());
            }
            http.post(&url).multipart(form)
        })
        .await
    }

    /// Authenticated GET returning the raw body, for endpoints that answer with CSV instead of JSON.
    pub(crate) async fn get_text(&self, url: String) -> Result<String, Error> {
        let res = self.execute(|http| http.get(&url)).await?;
        let status = res.status();
        let body = res.text().await?;
        if !status.is_success() {
            return Err(Error::from_response(status, body));
        }
        Ok(body)
    }

    /// Sends an authenticated request built by `build` and decodes the JSON response.
    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        build: impl Fn(&reqwest::Client) -> RequestBuilder,
    ) -> Result<T, Error> {
        decode(self.execute(build).await?).await
    }

    /// Sends an authenticated request built by `build`, retrying once after a re-login on `401`.
    async fn execute(
        &self,
        build: impl Fn(&reqwest::Client) -> RequestBuilder,
    ) -> Result<reqwest::Response, Error> {
        let token = self.bearer().await?;
        let res = build(&self.inner.http)
            .bearer_auth(&token)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;

        let res = if res.status() == StatusCode::UNAUTHORIZED && self.inner.credentials.is_some() {
            self.invalidate(&token).await;
            let token = self.bearer().await?;
            build(&self.inner.http)
                .bearer_auth(&token)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
        } else {
            res
        };

        Ok(res)
    }

    /// Sends an unauthenticated request (login).
    pub(crate) async fn send_unauthenticated<T: DeserializeOwned>(
        &self,
        req: RequestBuilder,
    ) -> Result<T, Error> {
        decode(
            req.header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?,
        )
        .await
    }

    async fn bearer(&self) -> Result<String, Error> {
        if let Some(t) = self.inner.token.read().await.as_ref()
            && (t.is_fresh() || self.inner.credentials.is_none())
        {
            return Ok(t.value.clone());
        }

        let _guard = self.inner.login_lock.lock().await;
        // Another task may have logged in while we waited for the lock.
        if let Some(t) = self.inner.token.read().await.as_ref()
            && t.is_fresh()
        {
            return Ok(t.value.clone());
        }
        if self.inner.credentials.is_none() {
            return Err(Error::MissingCredentials);
        }
        Ok(self.auth().login().await?.token)
    }

    async fn invalidate(&self, used: &str) {
        let mut token = self.inner.token.write().await;
        if token.as_ref().is_some_and(|t| t.value == used) {
            *token = None;
        }
    }
}

async fn decode<T: DeserializeOwned>(res: reqwest::Response) -> Result<T, Error> {
    let status = res.status();
    let body = res.text().await?;
    if !status.is_success() {
        return Err(Error::from_response(status, body));
    }
    if body.trim().is_empty() {
        // 202/204 with no body: accept `()`, `Option<_>`, `Value` or any all-optional struct.
        return serde_json::from_value(serde_json::Value::Null)
            .or_else(|_| serde_json::from_value(serde_json::json!({})))
            .map_err(|source| Error::Decode { source, body });
    }
    serde_json::from_str(&body).map_err(|source| Error::Decode { source, body })
}

/// Builder for [`Shiprocket`].
#[derive(Default, Debug)]
pub struct ShiprocketBuilder {
    credentials: Option<LoginRequest>,
    token: Option<String>,
    base_url: Option<String>,
    serviceability_base_url: Option<String>,
    http: Option<reqwest::Client>,
}

impl ShiprocketBuilder {
    /// API user email and password, used to obtain and refresh tokens.
    pub fn credentials(mut self, email: impl Into<String>, password: impl Into<String>) -> Self {
        self.credentials = Some(LoginRequest {
            email: email.into(),
            password: password.into(),
        });
        self
    }

    /// Initial token, e.g. one persisted from a previous run.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Override the API base url (no trailing slash), e.g. for a mock server.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Override the serviceability API base url (no trailing slash).
    pub fn serviceability_base_url(mut self, url: impl Into<String>) -> Self {
        self.serviceability_base_url = Some(url.into());
        self
    }

    /// Use a preconfigured `reqwest::Client` (timeouts, proxies, ...).
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    pub fn build(self) -> Shiprocket {
        let trim = |s: String| s.trim_end_matches('/').to_string();
        Shiprocket {
            inner: Arc::new(Inner {
                http: self.http.unwrap_or_default(),
                base_url: trim(self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.into())),
                serviceability_base_url: trim(
                    self.serviceability_base_url
                        .unwrap_or_else(|| DEFAULT_SERVICEABILITY_BASE_URL.into()),
                ),
                credentials: self.credentials,
                token: RwLock::new(self.token.map(|value| CachedToken {
                    value,
                    obtained_at: None,
                })),
                login_lock: Mutex::new(()),
            }),
        }
    }
}
