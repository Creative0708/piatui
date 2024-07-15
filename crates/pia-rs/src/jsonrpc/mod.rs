//! # Implementation of Private Internet Access's IPC Layer
//!
//! Private Internet Access uses JSON-RPC through a custom IPC layer. This layer allows for resynchronization in case a message is incompletely transmitted.
//!
//! This module is a rough translation of its C++ code to Rust. Go check out the [explanation](https://github.com/pia-foss/desktop/blob/522751571ea7f6b1a9e3dd5cc4c70fc2fd136221/common/src/ipc.cpp#L33) in the PIA repo for more details.

use async_std::io::{self};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub type PlatformDaemonConnection = unix::UnixSocketDaemonConnection;
    } else {
        compile_error!("platform not implemented D:");
    }
}

pub trait DaemonJSONRPCConnectionTrait: io::Read + io::Write + Clone {
    async fn new() -> io::Result<Self>;
}

const _: () = {
    const fn sanity_check<T: DaemonJSONRPCConnectionTrait>() {}
    sanity_check::<PlatformDaemonConnection>();
};

// Local socket magic number. always sent at the start of each frame
const PIA_LOCAL_SOCKET_MAGIC: [u8; 4] = 0xFFACCE56u32.to_be_bytes();

pub async fn connect() -> io::Result<(DaemonJSONRPCReciever, DaemonJSONRPCSender)> {
    let inner:  = PlatformDaemonConnection::new().await?;
    Ok(Self {
        reader: DaemonJSONRPCReciever::new(inner.clone()),
        writer: DaemonJSONRPCSender::new(inner),
    })
}

pub struct DaemonJSONRPCReciever(PlatformDaemonConnection);
impl DaemonJSONRPCReciever {
    fn new(reader: PlatformDaemonConnectionReader) -> Self {
        Self(reader)
    }
    fn  
}
pub struct DaemonJSONRPCSender(PlatformDaemonConnection);
impl DaemonJSONRPCSender {
    fn new(writer: PlatformDaemonConnectionWriter) -> Self {
        Self(writer)
    }
    fn  
}
