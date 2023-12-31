/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use async_trait::async_trait;

#[async_trait]
pub trait AsyncReceiver<T: Send + Sync> {
    type Error;

    async fn recv(&mut self) -> Result<T, Self::Error>;
}

pub trait Receiver<T> {
    type Error;

    fn recv(&mut self) -> Result<T, Self::Error>;
}

#[async_trait]
pub trait AsyncSender<T: Send + Sync>: Send {
    type Error;

    async fn send(&mut self, value: T) -> Result<(), Self::Error>;
}

pub trait Sender<T> {
    type Error;

    fn send(&mut self, value: T) -> Result<(), Self::Error>;
}

impl<T> Receiver<T> for std::sync::mpsc::Receiver<T> {
    type Error = std::sync::mpsc::RecvError;

    fn recv(&mut self) -> Result<T, Self::Error> {
        std::sync::mpsc::Receiver::recv(self)
    }
}

impl<T> Sender<T> for std::sync::mpsc::Sender<T> {
    type Error = std::sync::mpsc::SendError<T>;

    fn send(&mut self, value: T) -> Result<(), Self::Error> {
        std::sync::mpsc::Sender::send(self, value)
    }
}
#[async_trait]
impl<T: Send + Sync> AsyncReceiver<T> for tokio::sync::mpsc::Receiver<T> {
    type Error = tokio::sync::mpsc::error::TryRecvError;

    async fn recv(&mut self) -> Result<T, Self::Error> {
        tokio::sync::mpsc::Receiver::try_recv(self)
    }
}

#[async_trait]
impl<T: Send + Sync> AsyncSender<T> for tokio::sync::mpsc::Sender<T> {
    type Error = tokio::sync::mpsc::error::SendError<T>;

    async fn send(&mut self, value: T) -> Result<(), Self::Error> {
        tokio::sync::mpsc::Sender::send(self, value).await
    }
}

#[async_trait]
impl<T: Clone + Send + Sync> AsyncReceiver<T> for tokio::sync::broadcast::Receiver<T> {
    type Error = tokio::sync::broadcast::error::RecvError;

    async fn recv(&mut self) -> Result<T, Self::Error> {
        tokio::sync::broadcast::Receiver::recv(self).await
    }
}
