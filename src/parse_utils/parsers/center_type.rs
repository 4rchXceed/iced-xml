use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCenterTypeError {
    #[error("Invalid center type: {0}, expected align or center.")]
    InvalidCenterType(String),
}

/// Parse the center type value from a string into a boolean.
/// The center type value can be specified in the following format:
/// "align" or "center". If the value is "align", it will return true. If the value is "center", it will return false.
/// If the value is not one of these, it will default to false and print an error message.
pub fn parse_center_type(value: &str) -> Result<bool, ParseCenterTypeError> {
    return match value {
        "align" => Ok(true),
        "center" => Ok(false),
        _ => {
            return Err(ParseCenterTypeError::InvalidCenterType(value.to_string()));
        }
    };
}
