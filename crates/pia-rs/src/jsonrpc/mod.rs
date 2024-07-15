//! # Implementation of Private Internet Access's IPC Layer
//!
//! Private Internet Access uses JSON-RPC through a custom IPC layer. This layer allows for resynchronization in case a message is incompletely transmitted.
//!
//! This module is a rough translation of its C++ code to Rust. Go check out the [explanation](https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/common/src/ipc.cpp#L33) in the PIA repo for more details.

use std::mem::MaybeUninit;

use async_std::io::{self, ReadExt};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub type PlatformDaemonConnection = unix::UnixSocketDaemonConnection;
    } else {
        compile_error!("platform not implemented D:");
    }
}

pub trait DaemonJSONRPCConnectionTrait: io::Read + io::Write {
    async fn new() -> io::Result<Self>
    where
        Self: Sized;
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
    inner: PlatformDaemonConnection,
    last_server_seq_num: u16,
}

impl DaemonJSONRPCConnection {
    pub async fn new() -> io::Result<Self> {
        Ok(Self {
            inner: PlatformDaemonConnection::new().await?,
        })
    }

    pub async fn poll(&mut self) -> io::Result<Vec<u8>> {
        loop {}
    }

    async fn poll_raw(&mut self) -> io::Result<Vec<u8>> {
        // message header
        let mut header_buf = [0; 12];
        self.inner.read_exact(&mut header_buf);
        if header_buf[0..4] != PIA_LOCAL_SOCKET_MAGIC {
            return Result::Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "expected {PIA_LOCAL_SOCKET_MAGIC:?} as header magic number, got {:?}",
                    &header_buf[0..4]
                ),
            ));
        }
        let seq_low =
            u8::from_le_bytes((&header_buf[4..6]).try_into().expect("unreachable")) >> 4 as u8;
        let seq_high =
            u16::from_le_bytes((&header_buf[6..8]).try_into().expect("unreachable")) >> 4 as u8;
        let seq_num = seq_low as u16 | (seq_high as u16) << 8;

        if self.last_server_seq_num.wrapping_add(1) != seq_num {}
        self.last_server_seq_num = seq_num;

        let length = u32::from_le_bytes((&header_buf[8..12]).try_into().expect("unreachable"));
        if !VALID_MESSAGE_SIZES.contains(&length) {
            return Result::Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid length {:?} in header", length),
            ));
        }

        let mut buf = Vec::with_capacity(length as usize);
        buf.resize(length as usize, 0);
        self.inner.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
