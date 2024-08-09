use std::{mem::ManuallyDrop, sync::mpsc, thread::JoinHandle};

#[derive(Debug)]
pub struct DaemonConnection {
    pub rx: ManuallyDrop<mpsc::Receiver<ConnectionEvent>>,
    pub sender: pia_rs::DaemonConnectionSender,

    join_handles: ManuallyDrop<[JoinHandle<()>; 2]>,
}

pub enum ConnectionEvent {
    Daemon(pia_rs::event::DaemonEvent),
    Crossterm(ratatui::crossterm::event::Event),
}

impl DaemonConnection {
    pub fn take() -> Self {
        let (mut reciever, sender) = pia_rs::take_connection().unwrap();
        let (tx, rx) = mpsc::sync_channel(0);
        let tx2 = tx.clone();
        let join_handle_1 = std::thread::spawn(move || loop {
            let event = match reciever.poll() {
                Ok(event) => event,
                Err(err) => {
                    break;
                }
            };
            if tx.send(ConnectionEvent::Daemon(event)).is_err() {
                break;
            }
        });
        let join_handle_2 = std::thread::spawn(move || loop {
            let Ok(event) = ratatui::crossterm::event::read() else {
                break;
            };
            if tx2.send(ConnectionEvent::Crossterm(event)).is_err() {
                break;
            }
        });

        Self {
            rx: ManuallyDrop::new(rx),
            sender,

            join_handles: ManuallyDrop::new([join_handle_1, join_handle_2]),
        }
    }
}

impl Drop for DaemonConnection {
    fn drop(&mut self) {
        // SAFETY: self.rx/self.join_handle aren't dropped/moved anywhere else and they're not used after this call.
        unsafe {
            ManuallyDrop::drop(&mut self.rx);
            for join_handle in ManuallyDrop::take(&mut self.join_handles) {
                join_handle.join();
            }
        }
    }
}
