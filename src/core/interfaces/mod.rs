/*
    Appellation: interfaces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use async_trait::async_trait;

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
