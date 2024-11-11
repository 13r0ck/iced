use crate::futures;
use crate::graphics;
use crate::shell;
use crate::crossterm;

/// An error that occurred while running an application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The event loop executor could not be created.
    #[error("the event loop executor could not be created. Enable the 'crossterm' or 'crossterm-only' feature to use a TUI not a windowing GUI.")]
    EventLoopCreationFailed(Box<dyn std::error::Error + Send + Sync>),

    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed(Box<dyn std::error::Error + Send + Sync>),

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(graphics::Error),

    /// Temporary filler to make type system happy.
    #[cfg(feature = "crossterm")]
    #[error("Temporary filler to make type system happy.")]
    CrosstermIo(std::io::Error),
}

impl From<shell::Error> for Error {
    fn from(error: shell::Error) -> Error {
        match error {
            shell::Error::EventLoopCreationFailed(error) => {
                Error::EventLoopCreationFailed(Box::new(error))
            }
            shell::Error::ExecutorCreationFailed(error) => {
                Error::ExecutorCreationFailed(error)
            }
            shell::Error::WindowCreationFailed(error) => {
                Error::WindowCreationFailed(Box::new(error))
            }
            shell::Error::GraphicsCreationFailed(error) => {
                Error::GraphicsCreationFailed(error)
            }
        }
    }
}

#[cfg(feature = "crossterm")]
impl From<crossterm::Error> for Error {
    fn from(error: crossterm::Error) -> Error {
        match error {
            crossterm::Error::StdIo(e) => Error::CrosstermIo(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_send_sync() {
        fn _assert<T: Send + Sync>() {}
        _assert::<Error>();
    }
}
