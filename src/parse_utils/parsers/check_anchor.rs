use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCheckAnchorError {
    #[error("Invalid anchor: {0}, expected `top`, `bottom`, `left` or `right`")]
    InvalidAnchor(String),
}

/// Parse the anchor value from a string into an Iced Anchor enum.
/// The anchor value can be specified in the following format:
/// "top", "bottom", "left", "right"
pub fn check_anchor(value: &str) -> Result<String, ParseCheckAnchorError> {
    match value {
        "top" | "bottom" | "left" | "right" => Ok(value.to_string()),
        _ => {
            return Err(ParseCheckAnchorError::InvalidAnchor(value.to_string()));
        }
    }
}
