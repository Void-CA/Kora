// application/error.rs
// Unified error handling strategy for Kora's application layer
// Using thiserror for ergonomic error handling

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] Box<dyn std::error::Error + Send + Sync>),
    
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Unique constraint violated: {0}")]
    UniqueViolation(String),
}

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Rate error: {0}")]
    RateError(String),
    
    #[error("Not available: {0}")]
    NotAvailable(String),
}

// Convenience type alias for Results in application layer
pub type ApplicationResult<T> = Result<T, ApplicationError>;
