use async_trait::async_trait;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};

pub trait NBRead<T> {
    fn read(&mut self) -> &Option<T>;
}

#[async_trait]
pub trait Wait {
    async fn wait(&mut self) -> Result<(), ()>;
}

pub struct In<T> {
    signal: Receiver<T>,
    value: Option<T>,
}

impl<T> In<T> {
    pub fn new(signal: Receiver<T>) -> In<T> {
        In {
            signal,
            value: None,
        }
    }
}

impl<T: Clone> NBRead<T> for In<T> {
    fn read(&mut self) -> &Option<T> {
        match self.signal.try_recv() {
            Ok(val) => {
                self.value = Some(val.clone());
                &self.value
            }
            Err(_) => &self.value,
        }
    }
}

#[async_trait]
impl<T: Send> Wait for In<T> {
    async fn wait(&mut self) -> Result<(), ()> {
        let option = self.signal.recv().await;
        self.value = option;
        if let None = self.value {
            Err(())
        } else {
            Ok(())
        }
    }
}

pub struct Out<T> {
    signal: Sender<T>,
}

impl<T> Out<T> {
    pub fn new(signal: Sender<T>) -> Out<T> {
        Out { signal }
    }

    pub async fn write(&mut self, value: T) -> Result<(), SendError<T>> {
        self.signal.send(value).await
    }
}
