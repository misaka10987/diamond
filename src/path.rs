use std::path::PathBuf;

pub fn user_uds_path() -> anyhow::Result<PathBuf> {
    #[cfg(unix)]
    {
        use dirs::{data_local_dir, runtime_dir};

        let base = runtime_dir().or_else(data_local_dir).unwrap();

        let path = base.join("diamond.sock");
        Ok(path)
    }
    #[cfg(windows)]
    {
        use std::env::var;

        let user = var("USERNAME")?;
        let path = PathBuf::from("\\\\.\\pipe")
            .join("diamond-user")
            .join(user)
            .join("api.sock");
        Ok(path)
    }
}

pub fn system_uds_path() -> PathBuf {
    #[cfg(unix)]
    {
        "/var/run/diamond.sock".into()
    }
    #[cfg(windows)]
    {
        "\\\\.\\pipe\\diamond-system\\api.sock".into()
    }
}
