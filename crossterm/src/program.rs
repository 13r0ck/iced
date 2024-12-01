use iced_winit::{
    Program, graphics, Proxy,
    program::DefaultStyle,
};
use iced_graphics::Compositor;
use iced_core::window;
use crate::{Error, Settings};
use iced_futures::{Executor, Runtime};
use iced_runtime::user_interface::UserInterface;

use crossterm::{
    cursor, ExecutableCommand, QueueableCommand, style,
    terminal::{self, ClearType},
    style::Stylize,
};
use std::io::{stdout, Write};

/// The entry point of iced_crossterm
pub fn run<P, C>(
    settings: Settings,
    graphics_settings: graphics::Settings,
    window_settings: Option<window::Settings>,
    flags: P::Flags,
) -> Result<(), Error>
where
    P: Program + 'static,
    C: Compositor<Renderer = P::Renderer> + 'static,
    P::Theme: DefaultStyle,
{
  let mut stdout = stdout();

  let _ = stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        let _ = stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;


  let (program, task) = P::new(flags);
  let view = program.view(window::Id::unique());
  let cache = Cache::default();
  let renderer = Renderer::new();

  let user_interface = UserInterface::build(view, "foo", cache, renderer);
  println!("{:?}", program.view(window::Id::unique()));
  Ok(())
}
