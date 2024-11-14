use std::io::{stderr, IsTerminal};
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    init_logging();

    info!("Hello, world!");
}

fn init_logging() {
    let fmt = tracing_subscriber::fmt::Layer::default().with_ansi(stderr().is_terminal());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default().with(filter).with(fmt);

    tracing::subscriber::set_global_default(subscriber).expect("to set global subscriber");
}
