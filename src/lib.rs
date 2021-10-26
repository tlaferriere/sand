pub mod ports;
pub mod signals;
use async_trait::async_trait;

async fn wait() -> Result<(), ()> {
    Ok(())
}

#[async_trait]
pub trait Read<T> {
    fn nb_read(&mut self) -> Result<T, error::NBReadError>;
    async fn b_read(&mut self) -> Result<T, error::BReadError>;
}

#[async_trait]
pub trait Write<T> {
    fn nb_write(&self, val: T);
    async fn b_write(&self, val: T);
}

pub mod error {
    pub enum NBReadError {
        Empty,
        Closed,
    }

    pub enum BReadError {
        Closed,
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
