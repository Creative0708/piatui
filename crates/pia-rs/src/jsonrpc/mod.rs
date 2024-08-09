//! # Implementation of Private Internet Access's IPC Layer
//!
//! Private Internet Access uses JSON-RPC through a custom IPC layer. This layer allows for resynchronization in case a message is incompletely transmitted.
//!
//! This module is a rough translation of its C++ code to Rust. Go check out the [explanation](https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/common/src/ipc.cpp#L33) in the PIA repo for more details.

use std::{
    io::{self, Read},
    sync::atomic::{AtomicI32, AtomicU8, Ordering},
};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        use unix::create;
        type PlatformDaemonConnectionReader = unix::UnixSocketDaemonConnection;
        type PlatformDaemonConnectionWriter = unix::UnixSocketDaemonConnection;
    } else {
        compile_error!("platform not implemented D:");
    }
}

// Local socket magic number. always sent at the start of each frame
const PIA_LOCAL_SOCKET_MAGIC: [u8; 4] = 0xFFACCE56u32.to_be_bytes();

// Valid message sizes. Copied from PIA source
const VALID_MESSAGE_SIZES: std::ops::RangeInclusive<u32> = 2..=1024 * 1024;

struct ConnectionInfo {
    last_server_ack: AtomicI32,
    last_send_seq: AtomicI32,

    remaining: AtomicU8,
}

static CONNECTION_INFO: ConnectionInfo = ConnectionInfo {
    last_server_ack: AtomicI32::new(0),
    last_send_seq: AtomicI32::new(0),

    remaining: AtomicU8::new(0),
};

pub fn take_connection() -> io::Result<Option<(DaemonJSONRPCReceiver, DaemonJSONRPCSender)>> {
    if CONNECTION_INFO.remaining.load(Ordering::Acquire) != 0 {
        // connection still exists
        return Ok(None);
    }

    let (reader, writer) = create()?;

    CONNECTION_INFO.last_server_ack.store(-1, Ordering::Release);
    CONNECTION_INFO.last_send_seq.store(-1, Ordering::Release);

    Ok(Some((
        DaemonJSONRPCReceiver::new(reader),
        DaemonJSONRPCSender::new(writer),
    )))
}

pub struct DaemonJSONRPCReceiver {
    inner: PlatformDaemonConnectionReader,
}

impl DaemonJSONRPCReceiver {
    fn new(inner: PlatformDaemonConnectionReader) -> Self {
        Self { inner }
    }

    pub fn poll(&mut self) -> io::Result<Vec<u8>> {
        loop {
            let (seq_num, msg) = self.poll_raw()?;
            if msg.is_empty() {
                // message is an acknowledgement message; ignore
                CONNECTION_INFO
                    .last_server_ack
                    .store(seq_num as i32, Ordering::Release);
            } else {
                break Ok(msg);
            }
        }
    }

    /// Polls a message from the connection and returns the sequence number and its contents.
    fn poll_raw(&mut self) -> io::Result<(u16, Vec<u8>)> {
        // message header
        let mut header_buf = [0u32; 3];
        self.inner
            .read_exact(bytemuck::cast_slice_mut(&mut header_buf))?;
        if header_buf[0].to_ne_bytes() != PIA_LOCAL_SOCKET_MAGIC {
            return Result::Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "expected {PIA_LOCAL_SOCKET_MAGIC:?} as header magic number, got {:?}",
                    header_buf[0].to_ne_bytes()
                ),
            ));
        }
        let seq_shorts: [u16; 2] = bytemuck::cast(header_buf[1]);
        let seq_low = (u16::from_le_bytes(seq_shorts[0].to_ne_bytes()) >> 4) as u8;
        let seq_high = (u16::from_le_bytes(seq_shorts[1].to_ne_bytes()) >> 4) as u8;
        let seq_num = seq_low as u16 | (seq_high as u16) << 8;

        let length = u32::from_le_bytes(header_buf[2].to_ne_bytes());

        if length == 0 {
            return Ok((seq_num, vec![]));
        }

        if !VALID_MESSAGE_SIZES.contains(&length) {
            return Result::Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid length {:?} in header", length),
            ));
        }

        let mut buf = vec![0; length as usize];
        self.inner.read_exact(&mut buf)?;
        Ok((seq_num, buf))
    }
}

impl Drop for DaemonJSONRPCReceiver {
    fn drop(&mut self) {
        CONNECTION_INFO.remaining.fetch_sub(1, Ordering::Relaxed);
    }
}

pub struct DaemonJSONRPCSender {
    inner: PlatformDaemonConnectionWriter,
}
impl DaemonJSONRPCSender {
    fn new(inner: PlatformDaemonConnectionWriter) -> Self {
        Self { inner }
    }
}
