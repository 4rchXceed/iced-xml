use thiserror::Error;

use crate::css::complex_query::ComplexQueryError;

#[derive(Debug, Error)]
pub enum CssParseError {
    #[error("Unexpected end of input while parsing CSS at {0}")]
    UnexpectedEndOfInput(String),
    #[error("Invalid selector found while parsing CSS at {0}")]
    InvalidSelector(String),
    #[error("Cannot parse complex query inside another complex query: {0}")]
    ComplexQueryInComplexQuery(String),
    #[error("Error while parsing complex query '{0}': {1}")]
    ComplexQueryError(ComplexQueryError, String),
}
