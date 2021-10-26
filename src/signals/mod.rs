use async_trait::async_trait;

pub mod buffer;
pub mod signal;

#[async_trait]
pub(crate) trait Event {
    async fn event(&mut self);
}

struct Fifo {}
