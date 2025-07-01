use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::bail;
use interprocess::local_socket::{
    self, GenericFilePath, ListenerOptions, ToFsName, traits::tokio::Listener as _,
};
use tracing::{debug, error};

pub struct Listener {
    pub path: PathBuf,
    pub ipc: local_socket::tokio::Listener,
}

impl Listener {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        debug!("listening {path:?}");
        if path.exists() {
            bail!("{path:?} already exists");
        }
        let name = path.to_fs_name::<GenericFilePath>()?;
        let opt = ListenerOptions::new().name(name);
        let listener = opt.create_tokio()?;
        let val = Self {
            path: path.to_owned(),
            ipc: listener,
        };
        Ok(val)
    }
}

impl axum::serve::Listener for Listener {
    type Io = local_socket::tokio::Stream;

    type Addr = PathBuf;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        loop {
            let accept = self.ipc.accept().await;
            match accept {
                Ok(conn) => return (conn, self.path.clone()),
                Err(e) => {
                    error!("{e}");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    fn local_addr(&self) -> tokio::io::Result<Self::Addr> {
        Ok(self.path.clone())
    }
}
