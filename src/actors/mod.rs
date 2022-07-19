/*
    Appellation: actors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

mod api {
    use axum::{Router, Server};
    use std::net::SocketAddr;
    use tower_http::{
        compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
        sensitive_headers::SetSensitiveHeadersLayer, trace,
    };

    pub struct Context;

    pub fn create_server<C: Clone + std::marker::Send + std::marker::Sync + 'static>(
        address: SocketAddr,
        context: C,
    ) -> axum::Router {
        Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(
                http::header::HeaderName::from_static("x-request-id"),
            ))
            .layer(axum::Extension(context.clone()))
    }
}
