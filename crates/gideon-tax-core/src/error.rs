use thiserror::Error;

#[derive(Debug, Error)]
pub enum GideonTaxError {
    #[error("Value out of bounds: {0}")]
    OutOfBounds(String),
}
