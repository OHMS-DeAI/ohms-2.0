use thiserror::Error;

#[derive(Debug, Error)]
pub enum NovaQError {
    #[error("tensor has zero elements")]
    EmptyTensor,

    #[error("tensor dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch { expected: usize, found: usize },

    #[error("unable to compute statistics due to zero variance in column {column}")]
    ZeroVariance { column: usize },

    #[error("quantization requires at least {required} points, received {received}")]
    InsufficientSamples { required: usize, received: usize },

    #[error("k-means failed to converge within {iterations} iterations")]
    KMeansDidNotConverge { iterations: usize },

    #[error("configuration error: {0}")]
    InvalidConfig(String),

    #[error("internal invariant violated: {0}")]
    InvariantViolation(String),

    #[error("invalid input: {reason}")]
    InvalidInput { reason: String },
}

pub type Result<T> = std::result::Result<T, NovaQError>;
