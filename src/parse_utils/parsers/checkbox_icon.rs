use iced::{
    Font, Pixels,
    widget::{
        checkbox::Icon,
        text::{LineHeight, Shaping},
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCheckboxIconError {
    #[error("Invalid icon: {0}. Must be in the format: \"☑\" 20 absolute 15")]
    InvalidIcon(String),
    #[error("Error: {0} is not a number (f32). Must be in the format: \"☑\" 20 absolute 15")]
    NotANumber(String),
    #[error(
        "Invalid line-height-type: {0}. Must be in the format: \"☑\" 20 absolute 15, line-height-type must be either absolute or relative"
    )]
    InvalidLineHeightType(String),
}

/// Parse the checkbox icon value from a string into an Iced Checkbox Icon struct.
/// The icon value can be specified in the following format:
/// "<icon> <size> <line-height-type> <line-height-value>"
/// The icon is a single character (e.g. "☑") and must be enclosed in double quotes. The size is a number (e.g. 20) and is optional.
/// The line-height-type can be either "absolute" or "relative" and is required. The line-height-value is a number (e.g. 15) and is required.
pub fn parse_checkbox_icon(
    value: &str,
    font: &Font,
    shaping: Shaping,
) -> Result<Option<Icon<Font>>, ParseCheckboxIconError> {
    if value == "none" {
        return Ok(None);
    }

    if value.is_empty() {
        return Err(ParseCheckboxIconError::InvalidIcon(value.to_string()));
    }

    let parts: Vec<&str> = value.split(" ").collect();

    if parts.len() != 4 && parts.len() != 3 {
        return Err(ParseCheckboxIconError::InvalidIcon(value.to_string()));
    }

    let mut chars = parts
        .get(0)
        .ok_or(ParseCheckboxIconError::InvalidIcon(value.to_string()))?
        .chars();

    if chars.next() != Some('"') {
        return Err(ParseCheckboxIconError::InvalidIcon(value.to_string()));
    }

    let char = chars.next().unwrap_or('☑');

    if chars.next() != Some('"') {
        return Err(ParseCheckboxIconError::InvalidIcon(value.to_string()));
    }

    let mut i = 1;
    let size_str = parts[1].parse::<f32>();
    let mut size: Option<Pixels> = None;

    if let Ok(size_val) = size_str {
        size = Some(Pixels(size_val));
        i = 2;
    }

    if parts.len() <= i + 1 {
        return Err(ParseCheckboxIconError::InvalidIcon(value.to_string()));
    }

    let line_height = match parts[i] {
        "absolute" => LineHeight::Absolute(Pixels(
            parts[i + 1]
                .parse::<f32>()
                .map_err(|_| ParseCheckboxIconError::NotANumber(parts[i + 1].to_string()))?,
        )),
        "relative" => LineHeight::Relative(
            parts[i + 1]
                .parse::<f32>()
                .map_err(|_| ParseCheckboxIconError::NotANumber(parts[i + 1].to_string()))?,
        ),
        _ => LineHeight::Relative(10.0),
    };

    return Ok(Some(iced::widget::checkbox::Icon {
        font: font.clone(),
        code_point: char,
        size: size,
        line_height: line_height,
        shaping: shaping,
    }));
}
