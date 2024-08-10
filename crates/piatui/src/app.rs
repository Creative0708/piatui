use std::io;

use pia_rs::event::daemon::DaemonState;
use ratatui::{
    crossterm::{self, event::MouseButton},
    layout::Alignment,
    prelude::*,
    style::Stylize,
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

#[derive(Debug)]
pub struct App {
    conn: pia_rs::DaemonConnection,

    is_running: bool,
    state: Option<DaemonState>,
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
            let res = self.conn.poll();
            match res {
                Ok(e) => match *e {
                    pia_rs::event::daemon::DaemonEvent::Data([data]) => match self.state {
                        None => {
                            self.state = Some(serde_json::from_value(serde_json::Value::Object(
                                data.state.unwrap(),
                            ))?);
                        }
                        Some(ref mut state) => {
                            if let Some(new_state) = data.state {
                                let serde_json::Value::Object(mut object) =
                                    serde_json::to_value(&state)?
                                else {
                                    unreachable!();
                                };
                                for (key, value) in new_state {
                                    object[&key] = value;
                                }
                                *state = serde_json::from_value(serde_json::Value::Object(object))?;
                            }
                        }
                    },
                },
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                Err(err) => return Err(err),
            }
        }
        use crossterm::event;
        while event::poll(std::time::Duration::ZERO)? {
            match event::read()? {
                event::Event::Key(e)
                    if e.code == event::KeyCode::Char('c')
                        && e.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    self.is_running = false;
                }
                event::Event::Key(event::KeyEvent {
                    code: event::KeyCode::Char(' '),
                    ..
                }) => match self.state {
                    Some(DaemonState {
                        connection_state: pia_rs::event::daemon::ConnectionState::Disconnected,
                        ..
                    }) => {
                        self.conn
                            .send(pia_rs::event::client::ClientEvent::ConnectVPN)?;
                    }
                    Some(DaemonState {
                        connection_state: pia_rs::event::daemon::ConnectionState::Connected,
                        ..
                    }) => {
                        self.conn
                            .send(pia_rs::event::client::ClientEvent::DisconnectVPN)?;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(())
    }
}
impl Default for App {
    fn default() -> Self {
        let conn = pia_rs::take_connection().unwrap();
        Self {
            conn,
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
    state: Option<&'a DaemonState>,
}

impl Widget for MainInfo<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from("Main Info".bold());
        let block = Block::bordered().title(title.alignment(Alignment::Center));
        Paragraph::new(Text::from(vec![
            Line::from(vec![
                "Connection state: ".into(),
                self.state.map_or("...".into(), |state| {
                    use pia_rs::event::daemon::ConnectionState as CS;
                    let string = format!("{:?}", state.connection_state);
                    match state.connection_state {
                        CS::Disconnected => string.gray(),
                        CS::Connecting
                        | CS::Reconnecting
                        | CS::DisconnectingToReconnect
                        | CS::Disconnecting => string.yellow(),
                        CS::Connected => string.green(),
                        CS::Interrupted => string.red(),
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
