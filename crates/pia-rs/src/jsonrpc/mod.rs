use std::io::{self, Read};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub type PlatformDaemonConnection = unix::UnixSocketDaemonConnection;
    } else {
        compile_error!("platform not implemented D:");
    }
}

pub trait DaemonJSONRPCConnectionTrait: io::Read + io::Write {
    fn new() -> io::Result<Self>
    where
        Self: Sized;
}

const _: () = {
    const fn sanity_check<T: DaemonJSONRPCConnectionTrait>() {}
    sanity_check::<PlatformDaemonConnection>()
};

pub struct DaemonJSONRPCConnection {
    inner: PlatformDaemonConnection,
}

impl DaemonJSONRPCConnection {
    pub fn new() -> io::Result<Self> {
        let mut inner = PlatformDaemonConnection::new()?;

        let mut buf = [0; 1024];
        let mut size = inner.read(&mut buf)?;

        println!("{:?}", &buf[0..size]);

        todo!();

        // Ok(Self { inner })
    }
}
