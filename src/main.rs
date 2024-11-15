mod server;
pub mod error;

use std::io::{stderr, IsTerminal};
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use crate::error::Result;
use crate::server::{start_server, StartServerOptions};

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the client application
    Run(RunArgs)
}

#[derive(Debug, Parser)]
struct RunArgs {
    /// The port to run this client on.
    #[clap(short, long)]
    port: u16,
    /// The cookie to be used inside the node. This has to be the same
    /// value for all cluster nodes.
    #[clap(short, long)]
    cookie: String,
    /// The hostname for this node.
    #[clap(short='n', long)]
    hostname: String,
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_logging();

    match args.command {
        Command::Run(args) => {
            let (_, handle) = start_server(StartServerOptions {
                hostname: args.hostname,
                cookie: args.cookie,
                port: args.port,
            }).await?;

            info!("Starting server");

            handle.await?;
        }
    }

    Ok(())
}

fn init_logging() {
    let fmt = tracing_subscriber::fmt::Layer::default().with_ansi(stderr().is_terminal());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default().with(filter).with(fmt);

    tracing::subscriber::set_global_default(subscriber).expect("to set global subscriber");
}
