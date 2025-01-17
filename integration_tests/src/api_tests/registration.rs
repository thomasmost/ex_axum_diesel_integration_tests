use axum_test::transport_layer::IntoTransportLayer;
use axum_test::TestServer;

#[cfg(test)]
fn new_test_app<A>(app: A) -> TestServer
where
    A: IntoTransportLayer,
{
    TestServer::builder()
        .save_cookies()
        .expect_success_by_default()
        .mock_transport()
        .build(app)
        .unwrap()
}

#[cfg(test)]
mod registration_tests {

    use api_common::state::ServiceApplicationState;
    use axum::{routing::post, Router};
    use axum_test::TestServer;
    use data_access::db::Pool;
    use pretty_assertions::assert_eq;

    // Define a state struct
    #[derive(Clone)]
    struct AppState {
        pool: Pool,
    }

    #[tokio::test]
    async fn register_org() {
        let pool = data_access::integration_tests::common::db::get_pool();
        let state = ServiceApplicationState { pool: pool.clone() };

        // Set up the test server
        let app: Router<ServiceApplicationState> = Router::new()
            .route("/register_org", post(api_application::routes::register_org))
            .with_state(state);

        let server = TestServer::new(app).expect("Failed to start test server");

        let req = api_application::routes::OrgRegistrationRequest {
            name: "Test Org".to_string(),
        };

        let response = server
            .post("/register_org")
            .json(&serde_json::json!(req))
            .await;

        // Then
        assert_eq!(response.status_code(), http::StatusCode::CREATED);
    }
}
