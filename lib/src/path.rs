use std::path::PathBuf;

use thiserror::Error;

pub fn user_uds_path() -> Result<PathBuf, UserUdsPathError> {
    #[cfg(unix)]
    {
        use std::env::home_dir;

        let home = home_dir().ok_or(UserUdsPathError::MissingHome)?;
        let path = home
            .join(".local")
            .join("var")
            .join("run")
            .join("diamond.sock");
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

#[derive(Debug, Error)]
pub enum UserUdsPathError {
    #[cfg(unix)]
    #[error("no home directory for the user")]
    MissingHome,
    #[cfg(windows)]
    #[error(transparent)]
    Username(#[from] std::env::VarError),
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
