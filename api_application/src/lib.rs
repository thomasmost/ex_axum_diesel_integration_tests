use api_common::state::ServiceApplicationState;
use axum::{routing::post, Router};

pub mod routes;

pub fn configure() -> Router<ServiceApplicationState> {
    Router::new().route("/org", post(routes::register_org))
}
