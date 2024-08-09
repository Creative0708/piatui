use std::io;
mod app;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{terminal, ExecutableCommand},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn inner() -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        terminal::enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

        let mut app = app::App::default();

        while app.is_running() {
            terminal.draw(|frame| app.render_frame(frame))?;
            app.handle_events()?;
        }

        Ok(())
    }

    let res = inner();

    let mut stdout = io::stdout();
    stdout.execute(terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    res
}
