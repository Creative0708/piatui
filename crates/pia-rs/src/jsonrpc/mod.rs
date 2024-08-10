//! # Implementation of Private Internet Access's IPC Layer
//!
//! Private Internet Access uses JSON-RPC through a custom IPC layer. This layer allows for resynchronization in case a message is incompletely transmitted.
//!
//! This module is a rough translation of its C++ code to Rust. Go check out the [explanation](https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/common/src/ipc.cpp#L33) in the PIA repo for more details.

use std::{
    io::{self, Read, Write},
    sync::{
        atomic::{AtomicBool, AtomicU16, AtomicU8, Ordering},
        RwLock,
    },
};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        use unix::create;
        type PlatformDaemonConnectionReader = unix::UnixSocketDaemonConnectionReader;
        type PlatformDaemonConnectionWriter = unix::UnixSocketDaemonConnectionWriter;
    } else {
        compile_error!("platform not implemented D:");
    }
}

// Local socket magic number. always sent at the start of each frame
const PIA_LOCAL_SOCKET_MAGIC: [u8; 4] = 0xFFACCE56u32.to_be_bytes();

// Valid message sizes. Copied from PIA source
const VALID_MESSAGE_SIZES: std::ops::RangeInclusive<u32> = 2..=1024 * 1024;

pub struct ConnectionInfo {
    global: RwLock<GlobalConnectionInfo>,

    last_server_ack: AtomicU16,
    last_send_seq: AtomicU16,

    live: AtomicBool,
}

pub static CONNECTION_INFO: ConnectionInfo = ConnectionInfo {
    global: RwLock::new(GlobalConnectionInfo {
        jsonrpc_version: None,
    }),

    last_server_ack: AtomicU16::new(0),
    last_send_seq: AtomicU16::new(0),

    live: AtomicBool::new(false),
};

struct GlobalConnectionInfo {
    jsonrpc_version: Option<String>,
}

#[derive(Debug)]
pub enum TakeConnectionError {
    Io(io::Error),
    AlreadyTaken,
}
impl From<io::Error> for TakeConnectionError {
    fn from(inner: io::Error) -> Self {
        Self::Io(inner)
    }
}

pub fn take_connection() -> Result<DaemonJSONRPCConnection, TakeConnectionError> {
    if CONNECTION_INFO.live.load(Ordering::Acquire) {
        // connection still exists
        return Err(TakeConnectionError::AlreadyTaken);
    }

    let (reader, writer) = create()?;

    CONNECTION_INFO.last_server_ack.store(0, Ordering::Release);
    CONNECTION_INFO.last_send_seq.store(0, Ordering::Release);
    Ok(DaemonJSONRPCConnection::new(reader, writer))
}

#[derive(Debug)]
pub struct DaemonJSONRPCConnection {
    reader: PlatformDaemonConnectionReader,
    writer: PlatformDaemonConnectionWriter,
}

impl DaemonJSONRPCConnection {
    fn new(reader: PlatformDaemonConnectionReader, writer: PlatformDaemonConnectionWriter) -> Self {
        CONNECTION_INFO.live.store(true, Ordering::Release);
        Self { reader, writer }
    }

    pub fn poll(&mut self) -> io::Result<Vec<u8>> {
        loop {
            let (seq_num, msg) = self.poll_raw()?;
            if msg.is_empty() {
                // message is an acknowledgement message; ignore
                CONNECTION_INFO
                    .last_server_ack
                    .store(seq_num, Ordering::Release);
            } else {
                self.write_raw(seq_num, &[])?;
                break Ok(msg);
            }
        }
    }

    /// Polls a message from the connection and returns the sequence number and its contents.
    fn poll_raw(&mut self) -> io::Result<(u16, Vec<u8>)> {
        // message header
        let mut header_buf = [0; 12];
        self.read_exact(&mut header_buf, false)?;
        let header_buf: [u32; 3] = bytemuck::cast(header_buf);
        if header_buf[0].to_le_bytes() != PIA_LOCAL_SOCKET_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "expected {PIA_LOCAL_SOCKET_MAGIC:?} as header magic number, got {:?}",
                    header_buf[0].to_le_bytes()
                ),
            ));
        }
        let seq_shorts: [u16; 2] = bytemuck::cast(header_buf[1]);
        let seq_low = (seq_shorts[0].to_le() >> 4) as u8;
        let seq_high = (seq_shorts[1].to_le() >> 4) as u8;
        let seq_num = seq_low as u16 | (seq_high as u16) << 8;

        let length = header_buf[2].to_le();

        if length == 0 {
            return Ok((seq_num, vec![]));
        }

        if !VALID_MESSAGE_SIZES.contains(&length) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid length {:?} in header", length),
            ));
        }

        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf, true)?;
        Ok((seq_num, buf))
    }

    fn read_exact(&mut self, mut buf: &mut [u8], block: bool) -> io::Result<()> {
        let mut is_first_loop: bool = true;
        loop {
            let res = self.reader.read(buf);

            match res {
                Ok(0) => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
                Ok(read) => {
                    buf = &mut buf[read..];
                    if buf.is_empty() {
                        break;
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    if !block && is_first_loop {
                        return Err(err);
                    }
                    // other side is still writing; sleep for a bit
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
                Err(err) => return Err(err),
            }
            is_first_loop = false;
        }
        Ok(())
    }

    pub fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.write_raw(
            CONNECTION_INFO
                .last_send_seq
                .fetch_add(1, Ordering::SeqCst)
                .wrapping_add(1),
            bytes,
        )
    }
    fn write_raw(&mut self, seq_num: u16, bytes: &[u8]) -> io::Result<()> {
        // can't do .into() :pensive:
        const VALID_MESSAGE_SIZES_USIZE: std::ops::RangeInclusive<usize> =
            *VALID_MESSAGE_SIZES.start() as usize..=*VALID_MESSAGE_SIZES.end() as usize;

        assert!(
            bytes.is_empty() || VALID_MESSAGE_SIZES_USIZE.contains(&bytes.len()),
            "message is not an ack and its len {} is not in range of {:?}",
            bytes.len(),
            VALID_MESSAGE_SIZES_USIZE
        );

        // TODO: add checks for if the daemon is falling behind
        let [seq_low, seq_hi] = seq_num.to_le_bytes();

        self.writer.write_all(&PIA_LOCAL_SOCKET_MAGIC)?;
        self.writer
            .write_all(&((seq_low as u16) << 4).to_le_bytes())?;
        self.writer
            .write_all(&((seq_hi as u16) << 4).to_le_bytes())?;
        self.writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
        self.writer.write_all(bytes)?;
        self.writer.flush()?;

        Ok(())
    }
}

impl Drop for DaemonJSONRPCConnection {
    fn drop(&mut self) {
        CONNECTION_INFO.live.store(false, Ordering::Release);
    }
}
