use api_common::middleware::log_request;
use axum::body::Body;
use axum::routing::get;
use axum::Router;
use http::Request;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{info_span, Level};

use api_common::state::ServiceApplicationState;

use data_access::db::Pool;

use tracing::info;

pub mod service_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let pool = Pool::default();

    let app_state = ServiceApplicationState { pool };

    info!("Starting HTTP server...");
    let mut routes = Router::new().route("/", get(service_info::service_info));

    let application_routes = api_application::configure();

    routes = routes.merge(application_routes);

    routes = routes
        .layer(axum::middleware::from_fn(log_request))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // Get request id from the extensions...
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // ...and add it into info span
                info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // Note that this should be added *after* the Trace layer
        .layer(RequestIdLayer);

    info!("Listening on port 8000");

    let app = routes.with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
