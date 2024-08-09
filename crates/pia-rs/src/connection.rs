use std::io;

use crate::{event, jsonrpc};

pub use jsonrpc::TakeConnectionError;

#[derive(Debug)]
pub struct DaemonConnectionReceiver(jsonrpc::DaemonJSONRPCReceiver);
impl DaemonConnectionReceiver {
    fn new(inner: jsonrpc::DaemonJSONRPCReceiver) -> Self {
        Self(inner)
    }

    pub fn poll(&mut self) -> io::Result<event::DaemonEvent> {
        let bytes = self.0.poll()?;
        std::fs::write("/tmp/a.json", &bytes).unwrap();
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug)]
pub struct DaemonConnectionSender(jsonrpc::DaemonJSONRPCSender);
impl DaemonConnectionSender {
    fn new(inner: jsonrpc::DaemonJSONRPCSender) -> Self {
        Self(inner)
    }
}

pub fn take_connection(
) -> Result<(DaemonConnectionReceiver, DaemonConnectionSender), TakeConnectionError> {
    let (rx, tx) = jsonrpc::take_connection()?;
    Ok((
        DaemonConnectionReceiver::new(rx),
        DaemonConnectionSender::new(tx),
    ))
}
