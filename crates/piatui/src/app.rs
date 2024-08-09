use std::io;

use pia_rs::event::data::VPNState;
use ratatui::{
    crossterm,
    layout::Alignment,
    prelude::*,
    style::Stylize,
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

#[derive(Debug)]
pub struct App {
    is_running: bool,
    state: Option<VPNState>,
}
impl App {
    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn handle_events(&mut self) -> io::Result<()> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(e)
                if e.code == crossterm::event::KeyCode::Char('c')
                    && e.modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.is_running = false;
            }
            _ => (),
        }

        Ok(())
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            state: None,
        }
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        MainInfo {
            state: self.state.as_ref(),
        }
        .render(Rect::from((area.as_position(), Size::new(32, 16))), buf)
    }
}

struct MainInfo<'a> {
    state: Option<&'a VPNState>,
}

impl Widget for MainInfo<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from("Main Info".bold());
        let block = Block::bordered().title(title.alignment(Alignment::Center));
        Paragraph::new(format!(
            "Connection State: {}",
            self.state.map_or("...".to_string(), |state| format!(
                "{:?}",
                state.connection_state
            )),
        ))
        .block(block)
        .render(area, buf);
    }
}
