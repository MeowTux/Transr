use thiserror::Error;
use pyo3::exceptions::PyException;
use pyo3::PyErr;

#[derive(Error, Debug)]
pub enum TransRError {
    #[error("❌ Pipeline Error: {0}")]
    PipelineError(String),
    
    #[error("❌ System Error: {0}")]
    SystemError(String),
    
    #[error("❌ Network Error: {0}")]
    NetworkError(String),
    
    #[error("❌ Crypto Error: {0}")]
    CryptoError(String),
    
    #[error("❌ Math Error: {0}")]
    MathError(String),
    
    #[error("❌ IO Error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("❌ Parse Error: {0}")]
    ParseError(String),
    
    #[error("❌ Region Error: Feature '{0}' tidak tersedia dalam region ini")]
    RegionError(String),
    
    #[error("❌ Validation Error: {0}")]
    ValidationError(String),
    
    #[error("⚠️  Unknown Error: {0}")]
    Unknown(String),
}

impl From<TransRError> for PyErr {
    fn from(err: TransRError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

pub type TransRResult<T> = Result<T, TransRError>;
