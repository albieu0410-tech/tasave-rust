use tasave::{Error, TasaVE};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const RATE_PAYLOAD: &str = r#"{
    "bcv_usd": 104.23,
    "bcv_eur": 112.5,
    "parallel_usdt": 108.1,
    "parallel_buy": 107.5,
    "parallel_sell": 108.7,
    "confidence": 92.0,
    "verified": true,
    "checked_against": ["dolarvzla", "binance_p2p"],
    "valid_from": "2026-06-30T00:00:00-04:00",
    "valid_until": "2026-07-01T00:00:00-04:00",
    "next_expected_update": "2026-07-01T16:00:00-04:00",
    "next_business_day": "2026-07-02",
    "is_preliminary": false,
    "official_since": "2026-06-30T14:30:00-04:00",
    "published_at": "2026-06-30T14:30:00-04:00",
    "sources": ["bcv", "dolarvzla"],
    "consensus": true,
    "updated_at": "2026-07-01T09:00:00-04:00",
    "stale": false,
    "stale_since": null
}"#;

const PARALLEL_PAYLOAD: &str = r#"{
    "parallel_usdt": 108.1,
    "parallel_buy": 107.5,
    "parallel_sell": 108.7,
    "sources": ["binance_p2p"],
    "updated_at": "2026-07-01T09:00:00-04:00"
}"#;

const STATUS_PAYLOAD: &str = r#"{
    "status": "ok",
    "last_updated": "2026-07-01T09:00:00-04:00",
    "confidence": 92.0,
    "verified": true,
    "sources": ["bcv", "dolarvzla"],
    "is_preliminary": false,
    "stale": false
}"#;

const CONVERT_PAYLOAD: &str = r#"{
    "amount": 100.0,
    "from_currency": "USD",
    "to_currency": "VES",
    "result": 10423.0,
    "rate": 104.23,
    "source": "bcv",
    "rate_updated_at": "2026-07-01T09:00:00-04:00"
}"#;

const HISTORY_ENTRY_PAYLOAD: &str = r#"{
    "date": "2026-06-20",
    "bcv_usd": 100.0,
    "bcv_eur": 108.0,
    "parallel_usdt": 103.5,
    "confidence": 90.0,
    "sources": ["bcv"]
}"#;

#[tokio::test]
async fn rates_current_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/rates"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(RATE_PAYLOAD, "application/json"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let rate = client.rates().current().await.expect("request should succeed");

    assert_eq!(rate.bcv_usd, 104.23);
    assert!(rate.verified);
}

#[tokio::test]
async fn rates_parallel_sends_bearer_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/rates/parallel"))
        .and(header("Authorization", "Bearer tv_live_test"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(PARALLEL_PAYLOAD, "application/json"))
        .mount(&server)
        .await;

    let client = TasaVE::with_key("tv_live_test").with_base_url(server.uri());
    let parallel = client.rates().parallel().await.expect("request should succeed");

    assert_eq!(parallel.parallel_usdt, Some(108.1));
}

#[tokio::test]
async fn rates_parallel_without_key_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/rates/parallel"))
        .respond_with(ResponseTemplate::new(401).set_body_string("API key required"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let err = client.rates().parallel().await.expect_err("should fail without a key");

    match err {
        Error::Api { status, message } => {
            assert_eq!(status, 401);
            assert_eq!(message, "API key required");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn history_date_not_found_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/history/2020-01-01"))
        .respond_with(ResponseTemplate::new(404).set_body_string("No rate found for 2020-01-01"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let err = client.history().date("2020-01-01").await.expect_err("should 404");

    assert!(matches!(err, Error::Api { status: 404, .. }));
}

#[tokio::test]
async fn history_range_success() {
    let server = MockServer::start().await;
    let body = format!("[{HISTORY_ENTRY_PAYLOAD}]");
    Mock::given(method("GET"))
        .and(path("/v1/history"))
        .and(query_param("from", "2026-06-01"))
        .and(query_param("to", "2026-06-24"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let history = client
        .history()
        .range("2026-06-01", "2026-06-24")
        .await
        .expect("request should succeed");

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].bcv_usd, 100.0);
}

#[tokio::test]
async fn status_service_unavailable_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/status"))
        .respond_with(ResponseTemplate::new(503).set_body_string("No rate data available"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let err = client.status().await.expect_err("should be unavailable");

    assert!(matches!(err, Error::Api { status: 503, .. }));
}

#[tokio::test]
async fn status_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/status"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(STATUS_PAYLOAD, "application/json"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let status = client.status().await.expect("request should succeed");

    assert_eq!(status.status, "ok");
}

#[tokio::test]
async fn convert_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convert"))
        .and(query_param("amount", "100"))
        .and(query_param("from", "USD"))
        .and(query_param("to", "VES"))
        .and(query_param("source", "bcv"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(CONVERT_PAYLOAD, "application/json"))
        .mount(&server)
        .await;

    let client = TasaVE::new().with_base_url(server.uri());
    let result = client
        .convert()
        .amount(100.0)
        .from("USD")
        .to("VES")
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(result.result, 10423.0);
}

#[tokio::test]
async fn convert_missing_param_never_hits_network() {
    // No mock registered — if this made a request, wiremock would panic on an unexpected call.
    let server = MockServer::start().await;
    let client = TasaVE::new().with_base_url(server.uri());

    let err = client
        .convert()
        .amount(100.0)
        .from("USD")
        .send() // no .to(...)
        .await
        .expect_err("should fail before sending a request");

    assert!(matches!(err, Error::MissingParam("to")));
}
