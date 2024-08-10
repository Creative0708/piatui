use std::io;

use crate::{event, jsonrpc};

pub use jsonrpc::TakeConnectionError;

#[derive(Debug)]
pub struct DaemonConnection(jsonrpc::DaemonJSONRPCConnection);
impl DaemonConnection {
    fn new(inner: jsonrpc::DaemonJSONRPCConnection) -> Self {
        Self(inner)
    }

    pub fn poll(&mut self) -> io::Result<Box<event::daemon::DaemonEvent>> {
        let bytes = self.0.poll()?;
        let res: Result<event::JSONRPCMessage<event::daemon::DaemonEvent>, serde_json::Error> =
            serde_json::from_slice(&bytes);
        if res.is_err() {
            std::fs::write("/tmp/a.json", &bytes).unwrap();
        }
        Ok(res?.event)
    }

    pub fn send(&mut self, event: event::client::ClientEvent) -> io::Result<()> {
        let bytes = serde_json::to_vec(&event::JSONRPCMessage {
            jsonrpc_version: "2.0".to_owned(),
            event: Box::new(event),
        })?;
        self.0.write(&bytes)?;
        Ok(())
    }
}

pub fn take_connection() -> Result<DaemonConnection, TakeConnectionError> {
    let connection = jsonrpc::take_connection()?;
    Ok(DaemonConnection::new(connection))
}
