//! This module contains the [In] and [Out] ports that connect to signals.

use crate::error::{BReadError, NBReadError};
use crate::signals::{SignalRead, SignalWrite};
use crate::{Read, Write};
use async_trait::async_trait;

#[async_trait]
/// Wait trait
pub trait Wait {
    /// Wait until next delta-cycle.
    async fn wait(&mut self) -> Result<(), ()>;
}

// pub trait Connect<T> {
//     fn connect(tx: &dyn SignalWrite<T = T, SR = dyn SignalRead<T = T>>) -> Self;
// }

#[macro_export]
/// Connect signal to In port.
macro_rules! connect_in {
    ($signal: expr) => {{
        $crate::ports::In {
            signal: $crate::subscribe!($signal),
            value: None,
        }
    }};
}

#[macro_export]
/// Connect signal to Out port.
macro_rules! connect_out {
    ($signal: expr) => {{
        $crate::ports::Out {
            signal: Box::new($signal.clone()),
        }
    }};
}

/// This is a port for an incoming signals.
pub struct In<T> {
    pub(crate) signal: Box<dyn SignalRead<T = T> + Send>,
    pub(crate) value: Option<T>,
}

// impl<T: Clone + Send> Connect<T> for In<T> {
//     /// Connect the signal receiver to this port.
//     fn connect(tx: &dyn SignalWrite<T = T>) -> Self {
//         In {
//             signal: Box::new(tx.subscribe()),
//             value: None,
//         }
//     }
// }

#[async_trait]
impl<T: Clone + Send + Sync> Read for In<T> {
    type T = T;

    fn nb_read(&mut self) -> Result<Self::T, NBReadError> {
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

    async fn b_read(&mut self) -> Result<Self::T, BReadError> {
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
pub struct Out<T> {
    signal: Box<dyn SignalWrite<T = T> + Send>,
}

// impl<T> Connect<T> for Out<T> {
//     /// Connect the signal sender to this port.
//     fn connect(tx: &dyn SignalWrite<T = T>) -> Self {
//         Out { signal: tx.clone() }
//     }
// }

#[async_trait]
impl<T: Clone + Send + Sync> Write for Out<T> {
    type T = T;

    fn nb_write(&self, val: Self::T) {
        self.signal.nb_write(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::signal;
    use crate::signals::SignalRead;

    #[test]
    fn test_connect_in() {
        let tx: Sender<i32> = signal();
        let in_port = connect_in!(tx);
    }

    #[test]
    fn test_connect_out() {
        let tx: Sender<i32> = signal();
        let out_port = connect_out!(tx);
    }

    #[tokio::test]
    async fn test_out_write() {
        let test_val = 42;
        let tx = signal();
        let out = connect_out!(tx);
        let mut rx = subscribe!(tx);
        out.nb_write(test_val);
        assert_eq!(test_val, rx.nb_read().unwrap_or(0));
    }

    #[tokio::test]
    async fn test_in_nbread() {
        let test_val = 42;
        let tx = signal();
        let mut port_in = connect_in!(tx);
        tx.nb_write(test_val);
        assert_eq!(test_val, port_in.nb_read().unwrap_or(0));
    }
}
