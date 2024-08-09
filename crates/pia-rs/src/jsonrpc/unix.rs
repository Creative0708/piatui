use std::{
    io::{self, BufReader, BufWriter},
    os::unix::net::UnixStream,
};

pub fn create() -> io::Result<(
    UnixSocketDaemonConnectionReader,
    UnixSocketDaemonConnectionWriter,
)> {
    let socket = UnixStream::connect("/opt/piavpn/var/daemon.sock")?;
    Ok((BufReader::new(socket.try_clone()?), BufWriter::new(socket)))
}

pub type UnixSocketDaemonConnectionReader = BufReader<UnixStream>;
pub type UnixSocketDaemonConnectionWriter = BufWriter<UnixStream>;
