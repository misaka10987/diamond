mod listener;
mod server;

use clap::Parser;
use tokio::runtime;

use crate::server::Server;

#[derive(Parser)]
struct Diamond {
    /// Operate in system mode. User mode by default.
    #[arg(long, default_value_t = false)]
    system: bool,
}

fn main() -> anyhow::Result<()> {
    let arg = Diamond::parse();
    let run = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let server = Server::new(arg.system);
    run.block_on(server.run())?;
    Ok(())
}
