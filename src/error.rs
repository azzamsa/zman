use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InvalidArgument(String),

    #[error("No such time")]
    InvalidTime,
}

impl std::convert::From<jiff::Error> for Error {
    fn from(err: jiff::Error) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}
