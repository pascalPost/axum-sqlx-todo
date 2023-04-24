use axum::http::StatusCode;
use axum_test_helper::TestClient;
use rust_rest_api::app;

#[tokio::test]
async fn health_check() {
    let app = app().await.unwrap();
    let client = TestClient::new(app);
    let res = client.get("/health_check").send().await;
    assert_eq!(res.status(), StatusCode::OK);
}
