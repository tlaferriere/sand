use std::borrow::Borrow;
use std::sync::mpsc::{Receiver, RecvError, Sender, SendError, TryRecvError};

pub struct In<T> {
    signal: Receiver<T>,
    value: Option<T>
}

impl<T: Clone> In<T> {
    pub fn new(signal: Receiver<T>) -> In<T> {
        In {
            signal,
            value: None
        }
    }

    pub fn read(&mut self) -> &Option<T> {
        match self.signal.try_recv() {
            Ok(val) => {
                self.value = Some(val.clone());
                &self.value
            },
            Err(_) => &self.value
        }
    }

    pub fn wait(&mut self) -> Result<(), RecvError> {
        match self.signal.recv() {
            Ok(val) => {
                self.value = Some(val.clone());
                Ok(())
            }
            Err(err) => Err(err)
        }
    }
}

pub struct Out<T> {
    signal: Sender<T>,
}

impl<T> Out<T> {
    pub fn new(signal: Sender<T>) -> Out<T> { Out{signal} }

    pub fn write(&mut self, value: T) -> Result<(), SendError<T>> {
        self.signal.send(value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
