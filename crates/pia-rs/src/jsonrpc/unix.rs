use std::{io, os::unix::net::UnixStream};

pub fn create() -> UnixSocketDaemonConnection {
    
}

// too lazy for newtype
pub type UnixSocketDaemonConnection = UnixStream;

impl super::DaemonJSONRPCConnectionTrait for UnixSocketDaemonConnection {
    fn new() -> io::Result<Self> {
        Self::connect("/opt/piavpn/var/daemon.sock")
    }
}
