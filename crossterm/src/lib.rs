//! A terminal based alternative to the iced winowind shell.
//! Iced can run in one of three modes:
//!     1. No TUI
//!     2. TUI fallback if GUI fails
//!     3. TUI only


/// The entry point for iced_crossterm.
pub mod program;
pub mod error;
pub mod settings;

pub use error::Error;
pub use settings::Settings;
