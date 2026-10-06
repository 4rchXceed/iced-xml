use iced::{
    Font, Pixels,
    widget::text_input::{Icon, Side},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseSelectIconError {
    #[error("Invalid icon: {0}. Example: select-icon: \">\" 20 left 15")]
    InvalidIcon(String),
    #[error("Error: {0} is not a number (f32). Example: select-icon: \">\" 20 left 15")]
    NotANumber(String),
    #[error("Invalid icon part: {0}. Example: select-icon: \">\" 20 left 15")]
    InvalidIconPart(String),
    #[error(
        "Invalid icon keyword: {0} (must be either left or right). Example: select-icon: \">\" 20 left 15"
    )]
    InvalidIconKeyword(String),
}

/// Parse the select icon value from a string into an Iced Select Icon struct.
/// The icon value can be specified in the following format:
/// "<icon> <size> <side> <spacing>"
/// The icon is a single character (e.g. ">") and must be enclosed in double quotes.
/// The size is a number (e.g. 20) and is optional.
/// The side can be either "left" or "right" and is required.
/// The spacing is a number (e.g. 15) and is required.
pub fn parse_select_icon(
    value: &str,
    font: &Font,
) -> Result<Option<Icon<Font>>, ParseSelectIconError> {
    if value == "none" {
        return Ok(None);
    }

    if value.is_empty() {
        return Err(ParseSelectIconError::InvalidIcon(value.to_string()));
    }

    let parts: Vec<&str> = value.split(" ").collect();

    if parts.len() != 4 && parts.len() != 3 {
        return Err(ParseSelectIconError::InvalidIcon(value.to_string()));
    }

    let mut chars = parts
        .get(0)
        .ok_or(ParseSelectIconError::InvalidIconPart(value.to_string()))?
        .chars();

    if chars.next() != Some('"') {
        return Err(ParseSelectIconError::InvalidIconPart(value.to_string()));
    }

    let char = chars.next().unwrap_or('☑');

    if chars.next() != Some('"') {
        return Err(ParseSelectIconError::InvalidIconPart(value.to_string()));
    }
    let mut i = 1;
    let size_str = parts[1].parse::<f32>();
    let mut size: Option<Pixels> = None;

    if let Ok(size_val) = size_str {
        size = Some(Pixels(size_val));
        i = 2;
    }

    let side = match parts[i] {
        "left" => Side::Left,
        "right" => Side::Right,
        _ => {
            return Err(ParseSelectIconError::InvalidIconKeyword(
                parts[i].to_string(),
            ));
        }
    };

    let spacing_op = match parts[i + 1].parse::<f32>() {
        Ok(spacing_val) => spacing_val,
        Err(_) => {
            return Err(ParseSelectIconError::NotANumber(parts[i + 1].to_string()));
        }
    };

    return Ok(Some(iced::widget::text_input::Icon {
        font: font.clone(),
        code_point: char,
        size: size,
        side: side,
        spacing: spacing_op,
    }));
}
