use iced::Padding;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsePaddingError {
    #[error("Invalid padding: {0}")]
    InvalidPadding(String),
    #[error("Error: {0} is not a number (f32)")]
    NotANumber(String),
    #[error("Invalid padding part: {0}")]
    InvalidPaddingPart(String),
    #[error("Invalid padding keyword: {0}")]
    InvalidPaddingKeyword(String),
}

/// Parse a padding value from a string into an Iced Padding struct.
/// 4 possible errors:
/// - Invalid padding: if the string does not contain exactly 4 values separated by a space.
/// - Error: if any of the values is not a number (f32).
/// - Invalid padding part: if any of the values does not contain a valid keyword (top, right, bottom, left).
/// - Invalid padding keyword: if any of the values does not contain a valid keyword (top, right, bottom, left).
///  The padding can be specified in two ways:
///  - As a single value: `padding: 10;` (all sides will have the same padding)
///  - As a set of values: `padding: top=10 right=20 bottom=30 left=40;` (each side will have its own padding)
pub fn parse_padding(value: &String) -> Result<Padding, ParsePaddingError> {
    let value_float = value.parse::<f32>();

    if let Ok(val) = value_float {
        return Ok(Padding::new(val));
    }

    let value_sep = value.split(" ").collect::<Vec<&str>>();

    let mut top = 0.0;
    let mut right = 0.0;
    let mut bottom = 0.0;
    let mut left = 0.0;

    for (_, v) in value_sep.iter().enumerate() {
        let val_part = v.split("=").collect::<Vec<&str>>();
        if val_part.len() != 2 {
            return Err(ParsePaddingError::InvalidPaddingPart(v.to_string()));
        }
        let val_kw = val_part[0];

        let val_f32 = val_part[1]
            .parse()
            .map_err(|_| ParsePaddingError::NotANumber(val_part[1].to_string()))?;

        match val_kw {
            "top" => top = val_f32,
            "right" => right = val_f32,
            "bottom" => bottom = val_f32,
            "left" => left = val_f32,
            _ => return Err(ParsePaddingError::InvalidPaddingKeyword(val_kw.to_string())),
        }
    }
    return Ok(Padding {
        top: top,
        right: right,
        left: left,
        bottom: bottom,
    });
}
