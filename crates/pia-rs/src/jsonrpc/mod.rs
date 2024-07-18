//! # Implementation of Private Internet Access's IPC Layer
//!
//! Private Internet Access uses JSON-RPC through a custom IPC layer. This layer allows for resynchronization in case a message is incompletely transmitted.
//!
//! This module is a rough translation of its C++ code to Rust. Go check out the [explanation](https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/common/src/ipc.cpp#L33) in the PIA repo for more details.

use std::io::{self, Read};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub type PlatformDaemonConnectionReader = unix::UnixSocketDaemonConnection;
    } else {
        compile_error!("platform not implemented D:");
    }
}

const _: () = {
    const fn sanity_check<T: DaemonJSONRPCConnectionTrait>() {}
    sanity_check::<PlatformDaemonConnection>();
};

// Local socket magic number. always sent at the start of each frame
const PIA_LOCAL_SOCKET_MAGIC: [u8; 4] = 0xFFACCE56u32.to_be_bytes();

// Valid message sizes. Copied from PIA source
const VALID_MESSAGE_SIZES: std::ops::RangeInclusive<u32> = 2..=1024 * 1024;

pub struct DaemonJSONRPCConnection {
    pub receiver: DaemonJSONRPCReceiver,
    pub sender: DaemonJSONRPCSender,
}

pub struct DaemonJSONRPCReceiver {
    inner: PlatformDaemonConnection,
}

impl DaemonJSONRPCReceiver {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            inner: PlatformDaemonConnection::new()?,
            last_server_ack: None,
            last_sent_seq: None,
        })
    }

    pub fn poll(&mut self) -> io::Result<Vec<u8>> {
        loop {
            let (seq_num, msg) = self.poll_raw()?;
            if msg.is_empty() {
                // message is an acknowledgement message; ignore
                self.last_server_ack = Some(seq_num);
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
