use ractor::SpawnErr;
use thiserror::Error;
use tokio::task::JoinError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Spawn(#[from] SpawnErr),

    #[error(transparent)]
    Join(#[from] JoinError)
}