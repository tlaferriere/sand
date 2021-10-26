//! This module contains the [In] and [Out] ports that connect to signals.

use crate::error::{BReadError, NBReadError};
use crate::signal::{Receiver, Sender};
use crate::{Read, Write};
use async_trait::async_trait;

#[async_trait]
/// Wait trait
pub trait Wait {
    /// Wait until next delta-cycle.
    async fn wait(&mut self) -> Result<(), ()>;
}

/// This is a port for an incoming signals.
pub struct In<T: Clone + Send> {
    signal: Receiver<T>,
    value: Option<T>,
}

impl<T: Clone + Send> In<T> {
    /// Connect the signal receiver to this port.
    pub fn connect(rx: Receiver<T>) -> Self {
        In {
            signal: rx,
            value: None,
        }
    }
}

#[async_trait]
impl<T: Clone + Send + PartialEq> Read<T> for In<T> {
    fn nb_read(&mut self) -> Result<T, NBReadError> {
        match self.signal.nb_read() {
            Ok(val) => {
                self.value = Some(val.clone());
                Ok(val)
            }
            Err(err) => match err {
                NBReadError::Empty => match &self.value {
                    None => Err(err),
                    Some(val) => Ok(val.clone()),
                },
                NBReadError::Closed => Err(err),
            },
        }
    }

    async fn b_read(&mut self) -> Result<T, BReadError> {
        self.signal.b_read().await
    }

    async fn event(&mut self) {
        self.b_read().await;
    }
}

// #[async_trait]
// impl<T: Clone + Send> Wait for In<T> {
//     async fn wait(&mut self) -> Result<(), ()> {
//         self.value = match self.signal.nb_read() {
//             Ok(val) => {}
//             Err(_) => {}
//         };
//     }
// }

/// This is a port for outgoing signals.
pub struct Out<T: Clone + Send> {
    signal: Sender<T>,
}

impl<T: Clone + Send> Out<T> {
    /// Connect the signal sender to this port.
    pub fn connect(tx: Sender<T>) -> Self {
        Out { signal: tx }
    }
}

#[async_trait]
impl<T: Clone + Send> Write<T> for Out<T> {
    fn nb_write(&self, val: T) {
        self.signal.nb_write(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::signal;

    #[tokio::test]
    async fn test_out_write() {
        let test_val = 42;
        let (tx, mut rx) = signal();
        let out = Out::connect(tx);
        out.nb_write(test_val);
        assert_eq!(test_val, rx.nb_read().unwrap_or(0));
    }

    #[tokio::test]
    async fn test_in_nbread() {
        let test_val = 42;
        let (tx, rx) = signal();
        let mut port_in = In::connect(rx);
        tx.nb_write(test_val);
        assert_eq!(test_val, port_in.nb_read().unwrap_or(0));
    }
}
