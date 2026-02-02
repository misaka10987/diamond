mod listener;

use std::{ops::Deref, sync::Arc};

use crate::{system_uds_path, user_uds_path};
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use tokio::{fs::create_dir_all, signal::ctrl_c, sync::Notify};
use tracing::{debug, error, info};

use listener::Listener;

#[derive(Clone)]
pub struct Server(Arc<ServerInst>);

pub struct ServerInst {
    pub system: bool,
    pub shutdown: Notify,
}

impl Deref for Server {
    type Target = ServerInst;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Server {
    pub fn new(system: bool) -> Self {
        let val = ServerInst {
            system,
            shutdown: Notify::new(),
        };
        Self(Arc::new(val))
    }

    pub fn shutdown(&self) {
        info!("shutting down");
        self.shutdown.notify_waiters();
    }

    pub fn wait_shutdown(&self) -> impl Future<Output = ()> + Send + 'static {
        let server = self.clone();
        async move { server.shutdown.notified().await }
    }

    pub fn shutdown_on_ctrlc(&self) {
        let server = self.clone();
        tokio::spawn(async move {
            if let Err(e) = ctrl_c().await {
                error!("{e}");
            } else {
                server.shutdown();
            }
        });
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing_subscriber::fmt().init();
        let path = if self.system {
            system_uds_path()
        } else {
            user_uds_path()?
        };
        if let Some(p) = path.parent() {
            create_dir_all(p).await?;
        }
        let listener = Listener::new(path)?;
        let app = Router::new()
            .route("/exit", post(exit))
            .route("/version", get(env!("CARGO_PKG_VERSION")))
            .with_state(self.clone());
        self.shutdown_on_ctrlc();
        axum::serve(listener, app)
            .with_graceful_shutdown(self.wait_shutdown())
            .await?;
        Ok(())
    }
}

async fn exit(State(s): State<Server>) {
    debug!("received shutdown request");
    s.shutdown();
}
