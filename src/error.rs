use thiserror::Error;

use crate::parser::Rule;

pub type Result<T> = std::result::Result<T, FriggenError>;

#[derive(Debug, Error)]
pub enum FriggenError {
    #[error("friggenfile not found")]
    FriggenfileNotFound,

    #[error("friggenfile syntax error")]
    FriggenfileSyntaxError(Box<pest::error::Error<Rule>>),

    #[error("duplicate task definition: {0}")]
    DuplicateTaskDefinition(String),

    #[error("task not found: {0}")]
    TaskNotFound(String),

    #[error("invalid task reference: {referrer} » {referee}")]
    InvalidTaskReference { referrer: String, referee: String },

    #[error("cyclic task reference: {0:?}")]
    CyclicTaskReference(Vec<String>),

    #[error("io error: {source:?}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("error: {source:?}")]
    Error {
        #[from]
        source: anyhow::Error,
    },
}
