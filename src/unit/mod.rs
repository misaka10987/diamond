use std::{fs::File, process::Stdio, sync::OnceLock};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
    sync::Mutex,
};

#[serde_inline_default]
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
    /// Whether to write the `stdout`/`stderr` to a log file.
    #[serde_inline_default(false)]
    pub log_output: bool,
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

        let mut cmd = Command::new(&self.config.start[0]);
        cmd.args(&self.config.start[1..]);

        if self.config.log_output {
            // TODO: read the config to determine where to log
            let output = File::options()
                .write(true)
                .create(true)
                .open("/tmp/unit.log")?;
            cmd.stdout(output.try_clone()?).stderr(output);
        } else {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        };

        let proc = cmd.spawn()?;

        self.proc
            .lock()
            .await
            .set(proc)
            .expect("starting a unit for multiple times");
        Ok(())
    }

    /// Send an input to the process's `stdin`. This immediately flushes the input, so the process will receive it as soon as possible.
    ///
    /// Note that the argument `line` will be trimmed, and an LF will be automatically appended to it before sending.
    /// To write raw bytes directly, use [`Self::input`] instead.
    pub async fn input_line(&self, line: &str) -> anyhow::Result<()> {
        let line = format!("{}\n", line.trim());
        self.input(line.as_bytes()).await
    }

    pub async fn input(&self, buf: &[u8]) -> anyhow::Result<()> {
        let mut proc = self.proc.lock().await;
        let proc = proc.get_mut().ok_or(anyhow!("unit not started"))?;
        let stdin = proc.stdin.as_mut().ok_or(anyhow!("unit has no stdin"))?;
        stdin.write_all(buf).await?;
        stdin.flush().await?;
        Ok(())
    }
}
