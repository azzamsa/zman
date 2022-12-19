use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InvalidArgument(String),
}

impl std::convert::From<time::error::IndeterminateOffset> for Error {
    fn from(err: time::error::IndeterminateOffset) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<time::error::ComponentRange> for Error {
    fn from(err: time::error::ComponentRange) -> Self {
        Self::InvalidArgument(err.to_string())
    }
}
