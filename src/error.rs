use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse port.")]
    PortError,
    #[error(transparent)]
    ParseError(#[from] ParseError),
}

