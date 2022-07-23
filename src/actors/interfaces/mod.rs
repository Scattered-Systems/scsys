/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use api::*;
pub use cli::*;
pub use interface::*;

mod interface;

mod api {
    use async_trait::async_trait;
    use tower_http::{
        compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
        sensitive_headers::SetSensitiveHeadersLayer, trace,
    };

    /// An asynchronous trait designed to simplify the creation of Axum APIs
    #[async_trait]
    pub trait ApiSpec<Client = axum::Router> {
        async fn client(&self) -> Result<Client, crate::BoxError>
            where
                Self: Sized;
        fn constructor(&self) -> Result<Self, crate::BoxError>
            where
                Self: Sized;
        async fn run(&self) -> Result<(), crate::BoxError>
            where
                Self: Sized;
    }

    pub fn create_router<C: Clone + std::marker::Send + std::marker::Sync + 'static>(
        context: C,
    ) -> axum::Router {
        axum::Router::new()
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

mod cli {
    /// Scaffold the framework for the swift implementation of a CLI application built on top of clap
    pub trait CliSpec<Args, Conf, Data, Cont> {
        fn arguments(&self) -> Result<Args, crate::BoxError>
            where
                Self: Sized;
        fn constructor(&self) -> Result<Self, crate::BoxError>
            where
                Self: Sized;
        fn context(&self, settings: Conf) -> Result<Cont, crate::BoxError>
            where
                Self: Sized;
        fn data(&self) -> Result<Vec<Data>, crate::BoxError>
            where
                Self: Sized;
    }
}
