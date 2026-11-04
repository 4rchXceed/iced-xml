use iced::{Pixels, widget::text::LineHeight};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseLineHeightError {
    #[error("Invalid line height: {0}")]
    InvalidLineHeight(String),
    #[error("Error: {0} is not a number (f32)")]
    NotANumber(String),
}

/// Parse the line height value from a string into an Iced LineHeight enum.
/// The line height value can be specified in the following format:
/// "absolute <nbr>" or "relative <nbr>"
pub fn parse_line_height(value: &str) -> Result<LineHeight, ParseLineHeightError> {
    let split = value.split(" ").collect::<Vec<&str>>();

    if split.len() != 2 {
        return Err(ParseLineHeightError::InvalidLineHeight(value.to_string()));
    }

    let first = split
        .get(0)
        .ok_or(ParseLineHeightError::InvalidLineHeight(value.to_string()))?;
    let second = split
        .get(1)
        .ok_or(ParseLineHeightError::InvalidLineHeight(value.to_string()))?;

    return match *first {
        "absolute" => Ok(LineHeight::Absolute(Pixels(
            second
                .parse::<f32>()
                .map_err(|_| ParseLineHeightError::NotANumber(second.to_string()))?,
        ))),
        "relative" => {
            Ok(LineHeight::Relative(second.parse::<f32>().map_err(
                |_| ParseLineHeightError::NotANumber(second.to_string()),
            )?))
        }
        _ => {
            return Err(ParseLineHeightError::InvalidLineHeight(value.to_string()));
        }
    };
}
