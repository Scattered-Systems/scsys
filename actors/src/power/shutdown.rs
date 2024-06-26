/*
    Appellation: shutdown <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "tokio")]

use super::Power;
use tokio::sync::broadcast;

/// Listens for the server shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, the server
/// should shutdown.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct Shutdown {
    notify: broadcast::Receiver<()>,
    /// `true` if the shutdown signal has been received
    shutdown: Power,
}

impl Shutdown {
    /// Create a new `Shutdown` backed by the given `broadcast::Receiver`.
    pub fn new(notify: broadcast::Receiver<()>) -> Self {
        Shutdown {
            notify,
            shutdown: Power::On,
        }
    }

    /// Returns `true` if the shutdown signal has been received.
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.is_off()
    }

    /// Receive the shutdown notice, waiting if necessary.
    pub async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown.is_off() {
            return;
        }

        // Cannot receive a "lag error" as only one value is ever sent.
        let _ = self.notify.recv().await;

        // Remember that the signal has been received.
        self.shutdown = Power::off();
    }
}
