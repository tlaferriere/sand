#![deny(missing_docs)]
//! VLSI and embedded algorithms simulation crate.
//!
//! This crate is inspired by SystemC, but does not follow it.

pub mod ports;
mod signals;
use async_trait::async_trait;
pub use signals::buffer;
pub use signals::fifo;
pub use signals::signal;

/// Wait for a signal on the sensitivity list to trigger an event.
async fn wait() -> Result<(), ()> {
    Ok(())
}

/// A trait for reading from a signal.
#[async_trait]
pub trait Read<T> {
    /// Reads the value on the signal after it has been changed.
    ///
    /// The possible error values is:
    /// - [`NBReadError::Empty`]
    /// - [`NBReadError::Closed`]
    ///
    /// *This adds the read signal on the dynamic sensitivity list.*
    fn nb_read(&mut self) -> Result<T, error::NBReadError>;

    /// Read the value currently on the signal.
    ///
    /// This returns the value of the signal at the start of the current *delta-cycle*).
    /// The possible error values are
    /// - [`BReadError::Closed`]
    ///
    /// *This does **not** add the read signal on the dynamic sensitivity list.*
    async fn b_read(&mut self) -> Result<T, error::BReadError>;

    /// Suspend the process until a change event is detected on the signal.
    async fn event(&mut self);
}

/// A trait for writing to a signal.
#[async_trait]
pub trait Write<T> {
    /// Write a value to the signal.
    ///
    /// This returns immediately, but the signal is propagated at the next *delta-cycle*.
    fn nb_write(&self, val: T);
}

/// Various simulation errors.
pub mod error {
    /// Non blocking read errors:
    /// - [`NBReadError::Empty`]
    /// - [`NBReadError::Closed`]
    pub enum NBReadError {
        /// When the signal has never been written to, it is in an undefined state.
        Empty,
        /// When all the signal [Sender]s are dropped, the signal becomes closed. This is useful to
        /// end the simulation.
        Closed,
    }

    /// Blocking read errors:
    /// - [`BReadError::Closed`]
    pub enum BReadError {
        /// When all the signal [Sender]s are dropped, the signal becomes closed. This is useful to
        /// end the simulation.
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
