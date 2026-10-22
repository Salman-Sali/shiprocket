use serde_json::json;
use shiprocket::{Error, Shiprocket};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn login_response(token: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "id": 1, "first_name": "API", "last_name": "User", "email": "api@example.com",
        "company_id": 42, "created_at": "2024-01-01 10:00:00", "token": token
    }))
}

fn wallet_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({ "data": { "balance_amount": "-291539.83" } }))
}

async fn client(server: &MockServer) -> Shiprocket {
    Shiprocket::builder()
        .credentials("api@example.com", "secret")
        .base_url(server.uri())
        .build()
}

#[tokio::test]
async fn logs_in_lazily_and_reuses_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/external/auth/login"))
        .and(body_json(
            json!({ "email": "api@example.com", "password": "secret" }),
        ))
        .respond_with(login_response("tok-1"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/external/account/details/wallet-balance"))
        .and(header("authorization", "Bearer tok-1"))
        .respond_with(wallet_response())
        .expect(2)
        .mount(&server)
        .await;

    let client = client(&server).await;
    let first = client.account().wallet_balance().await.unwrap();
    assert_eq!(first.data.unwrap().balance_amount, Some(-291539.83));
    client.account().wallet_balance().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("tok-1"));
}

#[tokio::test]
async fn relogs_in_once_on_401() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/external/auth/login"))
        .respond_with(login_response("fresh"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/external/account/details/wallet-balance"))
        .and(header("authorization", "Bearer stale"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "message": "Token has expired", "status_code": 401
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/external/account/details/wallet-balance"))
        .and(header("authorization", "Bearer fresh"))
        .respond_with(wallet_response())
        .expect(1)
        .mount(&server)
        .await;

    let client = Shiprocket::builder()
        .credentials("api@example.com", "secret")
        .token("stale")
        .base_url(server.uri())
        .build();
    client.account().wallet_balance().await.unwrap();
    assert_eq!(client.token().await.as_deref(), Some("fresh"));
}

#[tokio::test]
async fn token_only_client_surfaces_401() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/external/account/details/wallet-balance"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "message": "Token has expired", "status_code": 401
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Shiprocket::builder()
        .token("stale")
        .base_url(server.uri())
        .build();
    let err = client.account().wallet_balance().await.unwrap_err();
    assert_eq!(err.status().map(|s| s.as_u16()), Some(401));
    assert!(matches!(err, Error::Api { ref message, .. } if message == "Token has expired"));
}

#[tokio::test]
async fn no_credentials_is_an_error() {
    let client = Shiprocket::builder().base_url("http://127.0.0.1:9").build();
    assert!(matches!(
        client.account().wallet_balance().await,
        Err(Error::MissingCredentials)
    ));
}

#[tokio::test]
async fn validation_errors_are_parsed() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/external/settings/company/addpickup"))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "message": "Not a valid mobile number",
            "errors": { "phone": ["The phone field is required."] },
            "status_code": 422
        })))
        .mount(&server)
        .await;

    let client = Shiprocket::builder()
        .token("t")
        .base_url(server.uri())
        .build();
    let req = shiprocket::pickup::AddPickupLocationRequest::builder()
        .pickup_location("Home")
        .name("Deadpool")
        .email("deadpool@chimichanga.com")
        .phone("")
        .address("Mutant Facility, Sector 3")
        .city("Pune")
        .state("Maharashtra")
        .country("India")
        .pin_code("110022")
        .build();
    match client.pickup().add(&req).await.unwrap_err() {
        Error::Api { status, errors, .. } => {
            assert_eq!(status.as_u16(), 422);
            assert_eq!(errors["phone"], vec!["The phone field is required."]);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn concurrent_requests_log_in_once() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/external/auth/login"))
        .respond_with(login_response("tok"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/external/account/details/wallet-balance"))
        .respond_with(wallet_response())
        .expect(8)
        .mount(&server)
        .await;

    let client = client(&server).await;
    let tasks: Vec<_> = (0..8)
        .map(|_| {
            let c = client.clone();
            tokio::spawn(async move { c.account().wallet_balance().await.unwrap() })
        })
        .collect();
    for t in tasks {
        t.await.unwrap();
    }
}
