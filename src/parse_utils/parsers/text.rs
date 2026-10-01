use iced::widget::text::{Shaping, Wrapping};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseShapingError {
    #[error("Invalid shaping: {0}")]
    InvalidShaping(String),
}

#[derive(Debug, Error)]
pub enum ParseTextWrapping {
    #[error("Invalid text wrapping: {0}")]
    InvalidTextWrapping(String),
}

/// Parse a shaping value from a string into an Iced Shaping enum.
/// Possible values: quality, performance, auto. If the value is not one of these, it will default to auto.
pub fn parse_shaping(value: &str) -> Result<Shaping, ParseShapingError> {
    match value {
        "quality" => Ok(Shaping::Advanced),
        "performance" => Ok(Shaping::Basic),
        "auto" => Ok(Shaping::Auto),
        _ => Err(ParseShapingError::InvalidShaping(value.to_string())),
    }
}

/// Parse the text wrapping value from a string into an Iced Wrapping enum.
/// Possible values: word, glyph, word-or-glyph, none. If the value is not one of these, it will default to none.
pub fn parse_text_wrapping(value: &str) -> Result<Wrapping, ParseTextWrapping> {
    match value {
        "word" => Ok(Wrapping::Word),
        "glyph" => Ok(Wrapping::Glyph),
        "word-or-glyph" => Ok(Wrapping::WordOrGlyph),
        "none" => Ok(Wrapping::None),
        _ => Err(ParseTextWrapping::InvalidTextWrapping(value.to_string())),
    }
}
