use crate::error::Result;
use ractor::{Actor, ActorRef};
use ractor::concurrency::JoinHandle;
use ractor_cluster::NodeServerMessage;

#[derive(Debug, Clone)]
pub(crate) struct StartServerOptions {
    pub port: u16,
    pub hostname: String,
    pub cookie: String,
}

pub(crate) async fn start_server(opts: StartServerOptions) -> Result<(ActorRef<NodeServerMessage>, JoinHandle<()>)> {
    let server = ractor_cluster::NodeServer::new(
        opts.port,
        opts.cookie,
        opts.hostname.clone(),
        opts.hostname,
        None,
        None
    );

    Ok(Actor::spawn(None, server, ()).await?)
}