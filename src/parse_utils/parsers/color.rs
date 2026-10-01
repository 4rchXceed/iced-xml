use iced::Color;
use thiserror::Error;

use crate::parse_utils::parsers::color_names::name_to_rgb;

#[derive(Debug, Error, Clone)]
pub enum ParseColorError {
    #[error("Invalid hex color: {0}")]
    InvalidHexColor(String),
    #[error("Invalid RGB color rgba(N,N,N): {0}")]
    InvalidRgbColor(String),
    #[error("Invalid RGBA color rgba(N,N,N,N): {0}")]
    InvalidRgbaColor(String),
    #[error("Color not found: {0}")]
    ColorNotFound(String),
    #[error("Hex values MUST have 6 characters (ex: #FF00AA). Alpha is *NOT* supported: {0}")]
    InvalidCharsNbrInHex(String),
}

/// Parse a color value from a string into an Iced Color enum.
/// 4 formats are supported:
/// - Hexadecimal: #RRGGBB or #RGB
/// - RGB: rgb(R, G, B)
/// - RGBA: rgba(R, G, B, A)
/// - Named colors: red, green, blue, etc. (see hex_rgb_converter crate)
pub fn parse_color_op(color: &String) -> Result<Option<Color>, ParseColorError> {
    if color.trim() == "none" || color.trim().is_empty() {
        return Ok(None);
    }

    return Ok(Some(parse_color(color)?));
}

pub fn parse_color(color: &String) -> Result<Color, ParseColorError> {
    if color.starts_with("#") {
        let color_clean = color
            .strip_prefix("#")
            .ok_or(ParseColorError::InvalidHexColor(color.clone()));

        let (r, g, b) = hex_to_rgb(color, color_clean)?;

        return Ok(Color::from_rgb(
            (r / 255) as f32,
            (g / 255) as f32,
            (b / 255) as f32,
        ));
    } else if color.starts_with("rgba") {
        let color_clean = color
            .strip_prefix("rgba(")
            .ok_or(ParseColorError::InvalidRgbaColor(color.clone()))?;

        let color_clean = color_clean
            .strip_suffix(")")
            .ok_or(ParseColorError::InvalidRgbaColor(color.clone()))?;

        let color_clean = color_clean.split(",").collect::<Vec<&str>>();

        if color_clean.len() != 4 {
            return Err(ParseColorError::InvalidRgbaColor(color.clone()));
        }

        let color_clean_f32: [f32; 4] = color_clean
            .iter()
            .map(|c| {
                c.trim()
                    .parse::<f32>()
                    .map_err(|_| ParseColorError::InvalidHexColor(color.clone()))
            })
            .collect::<Result<Vec<f32>, _>>()?
            .try_into()
            .map_err(|_| ParseColorError::InvalidRgbaColor(color.clone()))?;

        return Ok(Color::from_rgba(
            color_clean_f32[0] / 255.0,
            color_clean_f32[1] / 255.0,
            color_clean_f32[2] / 255.0,
            color_clean_f32[3],
        ));
    } else if color.starts_with("rgb") {
        let color_clean = color
            .strip_prefix("rgb(")
            .ok_or(ParseColorError::InvalidRgbColor(color.clone()))?;

        let color_clean = color_clean
            .strip_suffix(")")
            .ok_or(ParseColorError::InvalidRgbColor(color.clone()))?;

        let color_clean = color_clean.split(",").collect::<Vec<&str>>();

        if color_clean.len() != 3 {
            return Err(ParseColorError::InvalidRgbColor(color.clone()));
        }

        let color_clean_f32: [f32; 3] = color_clean
            .iter()
            .map(|c| {
                c.trim()
                    .parse::<f32>()
                    .map_err(|_| ParseColorError::InvalidHexColor(color.clone()))
            })
            .collect::<Result<Vec<f32>, _>>()?
            .try_into()
            .map_err(|_| ParseColorError::InvalidRgbaColor(color.clone()))?;

        return Ok(Color::from_rgb(
            color_clean_f32[0] / 255.0,
            color_clean_f32[1] / 255.0,
            color_clean_f32[2] / 255.0,
        ));
        // Special case for transparent
    } else if color == "transparent" {
        return Ok(Color::TRANSPARENT);
    } else {
        let (r, g, b) = name_to_rgb(color).ok_or(ParseColorError::ColorNotFound(color.clone()))?;

        return Ok(Color::from_rgb(
            (r / 255) as f32,
            (g / 255) as f32,
            (b / 255) as f32,
        ));
    }
}

fn hex_to_rgb(
    color: &String,
    color_clean: Result<&str, ParseColorError>,
) -> Result<(u32, u32, u32), ParseColorError> {
    let color_clean = color_clean.unwrap();
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    if color_clean.len() != 6 {
        return Err(ParseColorError::InvalidCharsNbrInHex(color.clone()));
    }
    for (i, c) in color_clean.chars().enumerate() {
        let value = c
            .to_digit(16)
            .ok_or(ParseColorError::InvalidHexColor(color.clone()))?;

        match i {
            0 => r = value,
            1 => r = (r << 4) + value,
            2 => g = value,
            3 => g = (g << 4) + value,
            4 => b = value,
            5 => b = (b << 4) + value,
            _ => return Err(ParseColorError::InvalidHexColor(color.clone())),
        }
    }
    Ok((r, g, b))
}
