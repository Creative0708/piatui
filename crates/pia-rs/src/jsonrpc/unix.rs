use std::{
    io::{self, BufReader, BufWriter},
    os::unix::net::UnixStream,
};

pub struct UnixSocketDaemonConnection(BufReader<UnixStream>, BufWriter<UnixStream>);

impl io::Read for UnixSocketDaemonConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_to_string(buf)
    }
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }
}
impl io::Write for UnixSocketDaemonConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.1.write(buf)
    }
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.1.write_vectored(bufs)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.1.flush()
    }
}

impl super::DaemonJSONRPCConnectionTrait for UnixSocketDaemonConnection {
    fn new() -> io::Result<Self> {
        let stream = UnixStream::connect("/opt/piavpn/var/daemon.sock")?;
        Ok(Self(
            BufReader::new(stream.try_clone()?),
            BufWriter::new(stream),
        ))
    }
}
