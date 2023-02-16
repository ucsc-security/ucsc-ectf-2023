use core::time::Duration;

pub mod lower_layers;
mod secure_uart;

pub use secure_uart::*;

/// Type definition for any [`CommunicationError`] [`Results`](core::result::Result).
pub type Result<T> = core::result::Result<T, CommunicationError>;

/// A channel to receive data from. See the documentation for [`recv`](RxChannel::recv) for
/// more info.
pub trait RxChannel {
    /// Receives data from the channel, putting the data received into ``dest``, returning the
    /// number of bytes written to it upon success. The buffer provided should have enough
    /// space to store the data that needs to be received along with its metadata size. After
    /// the timeout has passed, this function returns an error. Upon an error, a [`CommunicationError`]
    /// is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::RecvError`] - There are a couple of cases when this can occur:
    ///   - If this is a channel receiving communications from a
    ///     [`FramedTxChannel`](lower_layers::framing::FramedTxChannel), then this error could occur
    ///     if the provided buffer is too small to fit a whole message sent in a frame or if a malformed
    ///     message was sent.
    ///   - The timeout is reached.
    ///   - If this is a channel receiving communications from a channel in the crypto layer, such
    ///     as from an [`XChachaPoly1305Channel`](lower_layers::crypto::XChacha20Poly1305TxChannel)
    ///     then this error could occur if the provided buffer isn't big enough to store the additional
    ///     metadata, which can include a nonce and/or an authentication tag. Additionally, if the message
    ///     sent couldn't be authenticated, which can occur due to data corruption, then this error
    ///     will be returned.
    ///  - [`CommunicationError::InternalError`]
    ///    - This can occur if some internal error happens. This should only occur if something is wrong
    ///      with the implementation.
    fn recv(&mut self, dest: &mut [u8], timeout: Duration) -> Result<usize>;
}

/// A channel to send data through. See the documentation for [`send`](TxChannel::send) for
/// more info.
pub trait TxChannel {
    /// Sends the data from ``src`` through the channel. Upon an error, a [`CommunicationError`]
    /// is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::SendError`]
    ///   - This could occur if any implementation-based error occurs while sending data, such as too large
    /// of a message being sent.
    /// - [`CommunicationError::InternalError`]
    ///   - This can occur if some internal error happens. This should only occur if something is wrong
    ///     with the implementation.
    fn send(&mut self, src: &mut [u8]) -> Result<()>;
}

/// The possible errors that can occur while sending or receiving data through an [`RxChannel`] or a
/// [`TxChannel`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommunicationError {
    /// An error that can occur during a receive operation. See [RxChannel::recv] for more details.
    RecvError,

    /// An error that can occur during a send operation. See [TxChannel::send] for more details.
    SendError,

    /// An error that can occur if an internal error is encountered that should never happen.
    InternalError,
}
