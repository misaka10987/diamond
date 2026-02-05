use dirs::{data_local_dir, runtime_dir};
use std::{env::var, path::PathBuf};

pub fn user_uds_path() -> anyhow::Result<PathBuf> {
    if cfg!(windows) {
        let user = var("USERNAME")?;
        let path = PathBuf::from("\\\\.\\pipe")
            .join("diamond-user")
            .join(user)
            .join("api.sock");
        return Ok(path);
    }

    let base = runtime_dir().or_else(data_local_dir).unwrap();
    let path = base.join("diamond.sock");
    Ok(path)
}

pub fn system_uds_path() -> PathBuf {
    if cfg!(windows) {
        return "\\\\.\\pipe\\diamond-system\\api.sock".into();
    }
    "/var/run/diamond.sock".into()
}
