use iced::alignment::{Horizontal, Vertical};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseAlignError {
    #[error("Invalid align: {0}")]
    InvalidAlign(String),
}

/// Parse a horizontal alignment value from a string into an Iced Horizontal enum.
/// 3 possible values: start, center, end. If the value is not one of these, it will default to start.
pub fn parse_align_x(value: &String) -> Result<Horizontal, ParseAlignError> {
    match value.as_str() {
        "start" => Ok(Horizontal::Left),
        "center" => Ok(Horizontal::Center),
        "end" => Ok(Horizontal::Right),
        _ => Err(ParseAlignError::InvalidAlign(value.clone())),
    }
}

/// Parse a vertical alignment value from a string into an Iced Vertical enum.
/// 3 possible values: start, center, end. If the value is not one of these, it will default to start.
pub fn parse_align_y(value: &String) -> Result<Vertical, ParseAlignError> {
    match value.as_str() {
        "start" => Ok(Vertical::Top),
        "center" => Ok(Vertical::Center),
        "end" => Ok(Vertical::Bottom),
        _ => Err(ParseAlignError::InvalidAlign(value.clone())),
    }
}
