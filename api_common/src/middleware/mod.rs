use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use futures_util::StreamExt;

use tracing::info;

fn log_inbound() {
    info!("Inbound");
}
async fn log_outbound(parts: &http::response::Parts, body: Body) -> Body {
    if parts.status.is_success() {
        info!("Outcome {}", parts.status);
        return body;
    }
    let mut body_bytes = Vec::new();
    let mut data_stream = body.into_data_stream();
    while let Some(result) = data_stream.next().await {
        match result {
            Ok(bytes) => body_bytes.extend_from_slice(&bytes),
            Err(err) => {
                info!("Error reading body: {}", err);
                return Body::from_stream(data_stream);
            }
        }
    }

    let text = String::from_utf8_lossy(&body_bytes);
    info!("Outcome {} {:?}", parts.status, text);
    Body::from(body_bytes)
}

pub async fn log_request(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log_inbound();
    let res = next.run(req).await;

    let (res_parts, body) = res.into_parts();
    let body = log_outbound(&res_parts, body).await;
    let res = Response::from_parts(res_parts, body);

    Ok(res)
}
