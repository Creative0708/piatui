use async_std::{io, os::unix::net::UnixStream};

// too lazy for newtype
pub type UnixSocketDaemonConnection = UnixStream;

impl super::DaemonJSONRPCConnectionTrait for UnixSocketDaemonConnection {
    async fn new() -> io::Result<Self> {
        Self::connect("/opt/piavpn/var/daemon.sock").await
    }
}
