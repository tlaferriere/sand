use crate::error::{BReadError, NBReadError};
use crate::signals::Event;
use crate::{Read, Write};
use async_trait::async_trait;
use tokio::sync::broadcast;
use tokio::sync::broadcast::channel;
use tokio::sync::broadcast::error::{RecvError, TryRecvError};

#[derive(Clone)]
pub struct Sender<T> {
    tx: broadcast::Sender<T>,
}

impl<T: Clone + Send> Sender<T> {
    pub(crate) fn subscribe(&self) -> Receiver<T> {
        Receiver {
            rx: self.tx.subscribe(),
            value: None,
        }
    }
}

#[async_trait]
impl<T: Clone + Send> Write<T> for Sender<T> {
    fn nb_write(&self, val: T) {
        match self.tx.send(val) {
            Ok(_) => {}
            Err(_) => panic!("Unable to send on signal channel."),
        }
    }

    async fn b_write(&self, val: T) {
        todo!()
    }
}

pub struct Receiver<T: Clone + Send> {
    rx: broadcast::Receiver<T>,
    value: Option<T>,
}

#[async_trait]
impl<T: Clone + Send + PartialEq> Read<T> for Receiver<T> {
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
}

pub struct Signal<T: Clone + Send> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

#[async_trait]
impl<T: Clone + Send + Sync + PartialEq> Event for Signal<T> {
    async fn event(&mut self) {
        loop {
            match self.rx.rx.recv().await {
                Ok(val) => {
                    if let Some(old) = &self.rx.value {
                        if *old != val {
                            self.rx.value = Some(val);
                            return;
                        }
                    } else {
                        self.rx.value = Some(val);
                        return;
                    }
                }
                Err(err) => match err {
                    RecvError::Closed => {
                        return;
                    }
                    RecvError::Lagged(_) => {}
                },
            }
        }
    }
}

impl<T: Clone + Send> Signal<T> {
    pub fn new() -> Signal<T> {
        let (sender, rx) = channel(1);
        Signal {
            tx: Sender { tx: sender },
            rx: Receiver { rx, value: None },
        }
    }
}

impl<T: Clone + Send> Default for Signal<T> {
    fn default() -> Self {
        Signal::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_new() {
        let _: Signal<u32> = Signal::new();
    }

    #[test]
    fn test_signal_nb_read() {
        static TEST_VALUE: i32 = 42;
        let mut signal = Signal::new();
        signal.tx.tx.send(TEST_VALUE);
        assert_eq!(TEST_VALUE, signal.rx.nb_read().unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_b_read() {
        static TEST_VALUE: i32 = 42;
        let mut signal = Signal::new();
        signal.tx.tx.send(TEST_VALUE);
        assert_eq!(TEST_VALUE, signal.rx.b_read().await.unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_nb_write() {
        static TEST_VALUE: i32 = 42;
        let mut signal = Signal::new();
        signal.tx.nb_write(TEST_VALUE);
        assert_eq!(TEST_VALUE, signal.rx.rx.recv().await.unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_b_write() {
        static TEST_VALUE: i32 = 42;
        let mut signal = Signal::new();
        signal.tx.b_write(TEST_VALUE).await;
        assert_eq!(TEST_VALUE, signal.rx.rx.recv().await.unwrap_or(0));
    }

    #[tokio::test]
    async fn test_signal_change_event() {
        let mut signal = Signal::new();
        let tx = signal.tx.clone();
        tokio::task::spawn(async move {
            signal.event().await;
            assert_eq!(41, signal.rx.nb_read().unwrap_or(0));
            signal.event().await;
            assert_eq!(42, signal.rx.nb_read().unwrap_or(0));
        });
        tx.nb_write(41);
        tx.nb_write(42);
    }
}
