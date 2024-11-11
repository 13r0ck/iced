//! Configure your application.
use crate::{Font, Pixels};

use std::borrow::Cow;

/// The settings of an iced program.
#[derive(Debug, Clone)]
pub struct Settings {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,

    /// The default [`Font`] to be used.
    ///
    /// By default, it uses [`Family::SansSerif`](crate::font::Family::SansSerif).
    pub default_font: Font,

    /// The text size that will be used by default.
    ///
    /// The default value is `16.0`.
    pub default_text_size: Pixels,

    /// If set to true, the renderer will try to perform antialiasing for some
    /// primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// [`Canvas`], at a performance cost.
    ///
    /// By default, it is disabled.
    ///
    /// [`Canvas`]: crate::widget::Canvas
    pub antialiasing: bool,
   
    /// Configure when Iced should use the terminal UI, rather than the
    /// default windowing version.
    /// Requires enabling the iced feature "crossterm"
    /// See [`UseTUI`] for configuration.
    #[cfg(feature = "crossterm")]
    pub terminal: UseTUI,
}

#[cfg(feature = "crossterm")]
#[derive(Clone, Debug)]
/// Configuration for the terminal backend of iced when both winit and crossterm features are enabled.
pub enum UseTUI {
    /// Always use the default windowing GUI. Exit app if rendering the window fails for any
    /// reason. Default Iced behaviour.
    #[cfg(feature = "winit")]
    Never,
    /// Only use the terminal UI. Exit the app if rendering the terminal UI fails for any reason.
    Always,
    /// Try to run the GUI app first, then fallback to TUI if no display is available.
    /// This is useful to attempt to keep you app functioning via SSH or within a container that
    /// cannot create GUI windows on the host.
    #[cfg(feature = "winit")]
    WhenMissingDisplay,
    /// The opposite of [`UseTUI::WhenMissingDisplay`]. Try rendering the TUI first, and fallback
    /// to the default windowing UI if terminal does not support TUI.
    #[cfg(feature = "winit")]
    AlwaysWithWindowFallback,
}

#[cfg(not(feature = "crossterm"))]
impl Default for Settings {
    fn default() -> Self {
        Self {
            id: None,
            fonts: Vec::new(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: false,
        }
    }
}

#[cfg(feature = "crossterm")]
impl Default for Settings {
    fn default() -> Self {
        Self {
            id: None,
            fonts: Vec::new(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: false,
            terminal: UseTUI::Always,
        }
    }
}

#[cfg(feature = "winit")]
impl From<Settings> for iced_winit::Settings {
    fn from(settings: Settings) -> iced_winit::Settings {
        iced_winit::Settings {
            id: settings.id,
            fonts: settings.fonts,
        }
    }
}

#[cfg(feature = "crossterm")]
impl From<Settings> for iced_crossterm::Settings {
    fn from(settings: Settings) -> iced_crossterm::Settings {
        iced_crossterm::Settings {}
    }
}
