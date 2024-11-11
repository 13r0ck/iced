//! Crossterm errors.

#[derive(Debug)]
/// Crossterm errors
pub enum Error {
    /// A catch all for stdio errors
    StdIo(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::StdIo(error)
    }
}
