use std::{fs::File, sync::OnceLock};

use serde::{Deserialize, Serialize};
use tokio::{
    process::{Child, Command},
    sync::Mutex,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitConfig {
    /// Description.
    pub descr: String,
    /// Starting command of the unit.
    ///
    /// This accepts a list of strings as command and arguments.
    pub start: Vec<String>,
    #[serde(default)]
    pub deps: Vec<String>,
}

/// A running unit.
pub struct Unit {
    pub config: UnitConfig,
    proc: Mutex<OnceLock<Child>>,
}

impl Unit {
    /// Create a new unit to run with the given configuration.
    pub fn new(config: UnitConfig) -> Self {
        Self {
            config,
            proc: Mutex::new(OnceLock::new()),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        if !self.config.deps.is_empty() {
            async {}.await;
            todo!("ensure dependencies before starting the unit");
        }
        // TODO: read the config to determine where to log
        let output = File::options()
            .write(true)
            .create(true)
            .open("/tmp/unit.log")?;
        let child = Command::new(&self.config.start[0])
            .args(&self.config.start[1..])
            .stdout(output.try_clone()?)
            .stderr(output)
            .spawn()?;
        self.proc
            .lock()
            .await
            .set(child)
            .expect("starting a unit for multiple times");
        Ok(())
    }
}
