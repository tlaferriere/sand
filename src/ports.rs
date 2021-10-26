use crate::error::{BReadError, NBReadError};
use crate::signals::signal::{Receiver, Sender};
use crate::{Read, Write};
use async_trait::async_trait;

#[async_trait]
pub trait Wait {
    async fn wait(&mut self) -> Result<(), ()>;
}

// pub trait Connect<T> {
//     fn connect(tx: Sender<T>) -> Self;
// }

pub struct In<T: Clone + Send> {
    signal: Receiver<T>,
    value: Option<T>,
}

impl<T: Clone + Send> In<T> {
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

pub struct Out<T: Clone + Send> {
    signal: Sender<T>,
}

impl<T: Clone + Send> Out<T> {
    pub fn connect(tx: Sender<T>) -> Self {
        Out { signal: tx }
    }
}

#[async_trait]
impl<T: Clone + Send> Write<T> for Out<T> {
    fn nb_write(&self, val: T) {
        self.signal.nb_write(val);
    }

    async fn b_write(&self, val: T) {
        self.signal.b_write(val).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signals::signal::Signal;

    #[tokio::test]
    async fn test_out_write() {
        let test_val = 42;
        let mut signal = Signal::new();
        let out = Out::connect(signal.tx);
        out.nb_write(test_val);
        assert_eq!(test_val, signal.rx.nb_read().unwrap_or(0));
    }

    #[tokio::test]
    async fn test_in_nbread() {
        let test_val = 42;
        let signal = Signal::new();
        let mut port_in = In::connect(signal.rx);
        signal.tx.nb_write(test_val);
        assert_eq!(test_val, port_in.nb_read().unwrap_or(0));
    }
}
