use iced::{
    Background, Gradient, Radians,
    gradient::{ColorStop, Linear},
};
use thiserror::Error;

use crate::{
    parse_utils::parsers::color::{ParseColorError, parse_color},
    rs_utils::to_rad,
};

#[derive(Debug, Error, Clone)]
pub enum ParseBackgroundError {
    #[error(
        "Invalid linear gradient: linear gradiant must be in this format: linear-gradient(Ndeg, <color>,...) (max 8 colors). Currently: {0}"
    )]
    InvalidLinearGradient(String),
    #[error("Invalid color: {0}")]
    ColorParseError(ParseColorError),
}

/// Parse a background value from a string into an Iced Background enum.
/// The background value can be specified in the following format:
/// "linear-gradient(<angle>deg, <color1> <position1>%, <color2> <position2>%, ...)" or "<color>". The linear gradient can have a maximum of 8 colors.
/// Or it can be a single color value in any of the formats supported by the parse_color function.
/// If the value is not in the correct format, it will default to a transparent color and print an error message.
pub fn parse_background(value: &str) -> Result<Background, ParseBackgroundError> {
    let value = value.trim();

    if value.starts_with("linear-gradient(") && value.ends_with(")") {
        return parse_linear_gradiant(value);
    } else {
        return Ok(Background::Color(
            parse_color(&String::from(value))
                .map_err(|e| ParseBackgroundError::ColorParseError(e))?,
        ));
    }
}

/// Parses a linear gradiant in this format:
/// "linear-gradient(<angle>deg, <color1> <position1>%, <color2> <position2>%, ...)" or "<color>". The linear gradient can have a maximum of 8 colors.
fn parse_linear_gradiant(value: &str) -> Result<Background, ParseBackgroundError> {
    let error = ParseBackgroundError::InvalidLinearGradient(value.to_string());

    let value = value
        .strip_prefix("linear-gradient(")
        .ok_or(error.clone())?
        .strip_suffix(")")
        .ok_or(error.clone())?;

    let parts: Vec<&str> = value.split("deg").collect();

    let first = parts.get(0).ok_or(error.clone())?;

    let rotation_unparsed = first.trim();
    let rotation = rotation_unparsed
        .parse::<f32>()
        .map_err(|_| error.clone())?;
    let rotation_rad = to_rad(rotation);

    let value = value
        .split_once("deg")
        .ok_or(error.clone())?
        .1
        .trim()
        .strip_prefix(",")
        .ok_or(error.clone())?;

    let gradiant_stops: Vec<&str> = value
        .split("%")
        .map(|s| s.trim().strip_prefix(",").unwrap_or(s).trim_start())
        .filter(|s| !s.is_empty())
        .collect();

    let mut colors: [Option<ColorStop>; 8] = [None; 8];

    if gradiant_stops.len() > 8 {
        return Err(error.clone());
    }

    for (i, grandiant_stop) in gradiant_stops.iter().enumerate() {
        let grandiant_stop_parts = grandiant_stop
            .trim()
            .rsplit_once(char::is_whitespace)
            .ok_or(error.clone())?;

        let color = parse_color(&String::from(grandiant_stop_parts.0))
            .map_err(|e| ParseBackgroundError::ColorParseError(e))?;

        let position = grandiant_stop_parts
            .1
            .parse::<f32>()
            .map_err(|_| error.clone())?;

        colors[i] = Some(ColorStop {
            color: color,
            offset: position / 100.0,
        });
    }

    return Ok(Background::Gradient(Gradient::Linear(Linear {
        angle: Radians(rotation_rad),
        stops: colors,
    })));
}
