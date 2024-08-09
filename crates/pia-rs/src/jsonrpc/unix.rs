use std::{io, os::unix::net::UnixStream};

pub fn create() -> io::Result<(UnixSocketDaemonConnection, UnixSocketDaemonConnection)> {
    let socket = UnixStream::connect("/opt/piavpn/var/daemon.sock")?;
    Ok((socket.try_clone()?, socket))
}

// too lazy for newtype
pub type UnixSocketDaemonConnection = UnixStream;
