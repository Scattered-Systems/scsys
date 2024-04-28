/*
    Appellation: channel <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use async_trait::async_trait;

pub trait Receiver<T> {
    type Error;

    fn recv(&mut self) -> Result<T, Self::Error>;
}

pub trait Sender<T> {
    type Error;

    fn send(&mut self, value: T) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait AsyncReceiver<T> {
    type Error;

    async fn recv(&mut self) -> Result<T, Self::Error>;
}

#[async_trait]
pub trait AsyncSender<T>: Send {
    type Error;

    async fn send(&mut self, value: T) -> Result<(), Self::Error>;
}

#[cfg(feature = "std")]
mod std_sync_impl {
    use super::*;
    use std::sync::mpsc;

    impl<T> Receiver<T> for mpsc::Receiver<T> {
        type Error = mpsc::RecvError;

        fn recv(&mut self) -> Result<T, Self::Error> {
            mpsc::Receiver::recv(self)
        }
    }

    impl<T> Sender<T> for mpsc::Sender<T> {
        type Error = mpsc::SendError<T>;

        fn send(&mut self, value: T) -> Result<(), Self::Error> {
            mpsc::Sender::send(self, value)
        }
    }
}

#[cfg(feature = "tokio")]
mod tokio_impl {
    use super::*;
    use tokio::sync::mpsc;

    impl<T> Receiver<T> for mpsc::Receiver<T> {
        type Error = mpsc::error::TryRecvError;

        fn recv(&mut self) -> Result<T, Self::Error> {
            match mpsc::Receiver::blocking_recv(self) {
                Some(v) => Ok(v),
                None => Err(mpsc::error::TryRecvError::Empty),
            }
        }
    }

    macro_rules! impl_async_receiver {

        (@impl $($p:ident)::*.$call:ident -> $err:ty) => {

            #[async_trait::async_trait]
            impl<T> AsyncReceiver<T> for $($p)::*<T>
            where
                T: Send + Sync,
            {
                type Error = $err;

                async fn recv(&mut self) -> Result<T, Self::Error> {
                    $($p)::*::$call(self)
                }
            }
        };
    }

    impl_async_receiver!(@impl mpsc::Receiver.try_recv -> mpsc::error::TryRecvError);
}
