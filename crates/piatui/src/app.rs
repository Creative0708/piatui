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
    reciever: pia_rs::DaemonConnectionReceiver,
    sender: pia_rs::DaemonConnectionSender,

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
        loop {
            let res = self.reciever.poll();
            match res {
                Ok(e) => match *e.event {
                    pia_rs::event::DaemonEventInner::Data([data]) => {
                        self.state = Some(data.state);
                    }
                },
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                Err(err) => return Err(err),
            }
        }
        while crossterm::event::poll(std::time::Duration::ZERO)? {
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
        }

        Ok(())
    }
}
impl Default for App {
    fn default() -> Self {
        let (reciever, sender) = pia_rs::take_connection().unwrap();
        Self {
            reciever,
            sender,
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
        .render(Rect::from((area.as_position(), Size::new(64, 16))), buf)
    }
}

struct MainInfo<'a> {
    state: Option<&'a VPNState>,
}

impl Widget for MainInfo<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from("Main Info".bold());
        let block = Block::bordered().title(title.alignment(Alignment::Center));
        Paragraph::new(Text::from(vec![
            Line::from(vec![
                "Connection state: ".into(),
                self.state.map_or("...".into(), |state| {
                    let string = format!("{:?}", state.connection_state);
                    match state.connection_state {
                        pia_rs::event::data::ConnectionState::Disconnected => string.gray(),
                        pia_rs::event::data::ConnectionState::Connecting
                        | pia_rs::event::data::ConnectionState::Reconnecting
                        | pia_rs::event::data::ConnectionState::DisconnectingToReconnect
                        | pia_rs::event::data::ConnectionState::Disconnecting => string.yellow(),
                        pia_rs::event::data::ConnectionState::Connected => string.green(),
                        pia_rs::event::data::ConnectionState::Interrupted => string.red(),
                    }
                }),
            ]),
            Line::from(vec![
                "Public IP Address: ".into(),
                self.state
                    .map_or("...".into(), |state| match state.external_ip.0 {
                        Some(ip) => ip.to_string().green(),
                        None => "N/A".gray(),
                    }),
            ]),
            Line::from(vec![
                "VPN IP Address: ".into(),
                self.state
                    .map_or("...".into(), |state| match state.external_vpn_ip.0 {
                        Some(ip) => ip.to_string().green(),
                        None => "N/A".gray(),
                    }),
            ]),
        ]))
        .block(block)
        .render(area, buf);
    }
}
