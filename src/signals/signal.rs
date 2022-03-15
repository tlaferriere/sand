//! This module holds the basic signal.

use crate::error::{BReadError, NBReadError};
use crate::signals::{Connect, SignalRead, SignalWrite};
use crate::BoolEvent;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tokio::sync::broadcast::channel;
use tokio::sync::broadcast::error::{RecvError, TryRecvError};
use crate::port::{In, Out};

/// Wrapper for the `tokio::sync::braodcast::Sender`.
#[derive(Clone)]
pub struct Sender<T> {
    /// Detail
    pub tx: broadcast::Sender<T>,
}

#[async_trait]
impl<T: Clone + Send + PartialEq> SignalWrite for Sender<T> {
    type T = T;

    fn nb_write(&self, val: Self::T) {
        match self.tx.send(val) {
            Ok(_) => {}
            Err(_) => panic!("Unable to send on signal channel."),
        }
    }
}

impl<T: Clone + Send + PartialEq> Connect for Sender<T> {
    type T = T;

    fn connect_in<'a>(&'a mut self) -> In<Self::T> {
        In {
            signal: Box::new(Receiver { rx: self.tx.subscribe(), value: None }),
            value: None
        }
    }

    fn connect_out(&mut self) -> Out<Self::T> {
        todo!()
    }
}

/// Wrapper for `tokio::sync::broadcast::Receiver`.
///
/// This holds the previous received value in order to reflect the nature of an electrical signal.
pub struct Receiver<T> {
    pub(crate) rx: broadcast::Receiver<T>,
    pub(crate) value: Option<T>,
}

#[async_trait]
impl<T: Clone + Send + PartialEq> SignalRead for Receiver<T> {
    type T = T;

    fn nb_read(&mut self) -> Result<T, NBReadError> {
        match self.rx.try_recv() {
            Ok(val) => {
                self.value = Some(val.clone());
                Ok(val)
            }
            Err(err) => match err {
                TryRecvError::Empty => match self.value.clone() {
                    None => Err(NBReadError::Empty),
                    Some(val) => Ok(val),
                },
                TryRecvError::Closed => Err(NBReadError::Closed),
                TryRecvError::Lagged(_) => self.nb_read(),
            },
        }
    }

    async fn b_read(&mut self) -> Result<T, BReadError> {
        loop {
            match self.rx.recv().await {
                Ok(val) => {
                    if let Some(previous) = &self.value {
                        if *previous != val {
                            self.value = Some(val.clone());
                            return Ok(val);
                        }
                    } else {
                        self.value = Some(val.clone());
                        return Ok(val);
                    }
                }
                Err(err) => match err {
                    RecvError::Closed => return Err(BReadError::Closed),
                    RecvError::Lagged(_) => {}
                },
            }
        }
    }

    async fn event(&mut self) {
        self.b_read().await;
    }
}

#[async_trait]
impl BoolEvent for Receiver<bool> {
    /// Suspend the process until a positive edge event is detected on the signal.
    ///
    /// *This is only implemented for boolean value types.*
    async fn posedge_event(&mut self) {
        loop {
            if let Ok(val) = self.b_read().await {
                if val {
                    return;
                }
            } else {
                return;
            }
        }
    }

    /// Suspend the process until a negative edge event is detected on the signal.
    ///
    /// *This is only implemented for boolean value types.*
    async fn negedge_event(&mut self) {
        loop {
            if let Ok(val) = self.b_read().await {
                if !val {
                    return;
                }
            } else {
                return;
            }
        }
    }
}

/// Contructs a signal and returns the Sender handle.
pub fn signal<T: Clone>() -> Sender<T> {
    let (tx, _) = channel(1);
    Sender { tx }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let _: Sender<i32> = signal();
    }

    #[test]
    fn test_sender_subscribe() {
        let tx: Sender<i32> = signal();
        let _ = subscribe!(tx);
    }

    #[test]
    fn test_signal_nb_read() {
        static TEST_VALUE: i32 = 42;
        let tx = signal();
        let mut rx = subscribe!(tx);
        tx.tx.send(TEST_VALUE);
        assert_eq!(TEST_VALUE, rx.nb_read().unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_b_read() {
        static TEST_VALUE: i32 = 42;
        let tx = signal();
        let mut rx = subscribe!(tx);
        tx.tx.send(TEST_VALUE);
        assert_eq!(TEST_VALUE, rx.b_read().await.unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_nb_write() {
        static TEST_VALUE: i32 = 42;
        let tx = signal();
        let mut rx = subscribe!(tx);
        tx.nb_write(TEST_VALUE);
        assert_eq!(TEST_VALUE, rx.rx.recv().await.unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_event() {
        let tx = signal();
        let mut rx = subscribe!(tx);
        tokio::task::spawn(async move {
            rx.event().await;
            assert_eq!(41, rx.nb_read().unwrap_or(0));
            rx.event().await;
            assert_eq!(42, rx.nb_read().unwrap_or(0));
        });
        tx.nb_write(41);
        tx.nb_write(42);
    }

    #[tokio::test]
    async fn test_signal_posedge_event() {
        let tx = signal();
        let mut rx = subscribe!(tx);
        tokio::task::spawn(async move {
            rx.posedge_event().await;
            assert!(rx.nb_read().unwrap_or(false));
        });
        tx.nb_write(false);
        tx.nb_write(true);
    }

    #[tokio::test]
    async fn test_signal_negedge_event() {
        let tx = signal();
        let mut rx = subscribe!(tx);
        tokio::task::spawn(async move {
            rx.negedge_event().await;
            assert!(!rx.nb_read().unwrap_or(true));
        });
        tx.nb_write(true);
        tx.nb_write(false);
    }
}
