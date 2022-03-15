pub mod buffer;
pub mod fifo;
pub mod signal;
use crate::error;
use async_trait::async_trait;
use crate::port::{In, Out};

/// A trait for reading from a signal.
#[async_trait]
pub trait SignalRead {
    /// Type of the payload
    type T: Clone + Send;

    /// Reads the value on the signal after it has been changed.
    ///
    /// The possible error values is:
    /// - [`NBReadError::Empty`]
    /// - [`NBReadError::Closed`]
    ///
    /// *This adds the read signal on the dynamic sensitivity list.*
    fn nb_read(&mut self) -> Result<Self::T, error::NBReadError>;

    /// Read the value currently on the signal.
    ///
    /// This returns the value of the signal at the start of the current *delta-cycle*).
    /// The possible error values are
    /// - [`BReadError::Closed`]
    ///
    /// *This does **not** add the read signal on the dynamic sensitivity list.*
    async fn b_read(&mut self) -> Result<Self::T, error::BReadError>;

    /// Suspend the process until a change event is detected on the signal.
    async fn event(&mut self);
}

/// A trait for writing to a signal.
pub trait SignalWrite {
    /// Type of the payload
    type T: Clone + Send;

    /// Write a value to the signal.
    ///
    /// This returns immediately, but the signal is propagated at the next *delta-cycle*.
    fn nb_write(&self, val: Self::T);
}

/// A trait for writing to a signal.
pub trait Connect {
    /// Type of the payload
    type T: Clone + Send;

    /// Instantiate port instance connected to this signal.
    fn connect_in(&mut self) -> In<Self::T>;

    /// Instantiate port instance connected to this signal.
    fn connect_out(&mut self) -> Out<Self::T>;
}

