use thiserror::Error;

#[derive(Debug, Error)]
pub enum CssValParseError {
    #[error("{0}: {1}; -> Error (in value): {2}")]
    Value(String, String, String),
    #[error("{0}: {1}; -> Style option called {0} not found!")]
    KeyNotFound(String, String),
}
