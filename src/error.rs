use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum Error {
    #[error("Could not walk path: `{0}`")]
    WalkError(String),
}
