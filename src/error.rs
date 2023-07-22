use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InvalidArgument(String),

    #[error("No such time")]
    InvalidTime,
}
