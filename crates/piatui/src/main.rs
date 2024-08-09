use std::io;
mod app;
mod connection;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = app::App::default();

    while app.is_running() {
        terminal.draw(|frame| app.render_frame(frame))?;
        app.handle_events()?;
    }

    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    disable_raw_mode()?;

    Ok(())
}
