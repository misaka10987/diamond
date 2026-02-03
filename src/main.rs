pub mod path;
mod prelude;
mod server;
pub mod unit;

use clap::Parser;
pub use prelude::*;

#[derive(Clone, Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Parser)]
enum Command {
    Server {
        /// Operate in system mode. User mode by default.
        #[arg(long, default_value_t = false)]
        system: bool,
    },
}

fn main() {
    let args = Args::parse();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    match args.command {
        Command::Server { system } => {
            rt.block_on(Server::new(system).run()).unwrap();
        }
    }
}
