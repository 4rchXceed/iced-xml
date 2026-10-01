use iced::border::Radius;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseRadiusError {
    #[error("Invalid radius: {0}")]
    InvalidRadius(String),
    #[error("Error: {0} is not a number (f32)")]
    NotANumber(String),
    #[error("Invalid radius part: {0}")]
    InvalidRadiusPart(String),
    #[error("Invalid radius keyword: {0}")]
    InvalidRadiusKeyword(String),
}

/// Parse a radius value from a string into an Iced Radius struct.
/// 4 possible errors:
/// - Invalid radius: if the string does not contain exactly 4 values separated by a space.
/// - Error: if any of the values is not a number (f32).
/// - Invalid radius part: if any of the values does not contain a valid keyword (top_left, top_right, bottom_left, bottom_right).
/// - Invalid radius keyword: if any of the values does not contain a valid keyword (top_left, top_right, bottom_left, bottom_right).
///  The radius can be specified in two ways:
///  - As a single value: `radius: 10;` (all corners will have the same radius)
///  - As a set of values: `radius: top_left=10 top_right=20 bottom_left=30 bottom_right=40;` (each corner will have its own radius)
pub fn parse_radius(value: &String) -> Result<Radius, ParseRadiusError> {
    let value_float = value.parse::<f32>();
    if let Ok(val) = value_float {
        return Ok(Radius {
            bottom_left: val,
            top_right: val,
            bottom_right: val,
            top_left: val,
        });
    }
    let value_sep = value.split(" ").collect::<Vec<&str>>();
    let mut top_left = 0.0;
    let mut top_right = 0.0;
    let mut bottom_right = 0.0;
    let mut bottom_left = 0.0;
    for (_, v) in value_sep.iter().enumerate() {
        let val_part = v.split("=").collect::<Vec<&str>>();
        if val_part.len() != 2 {
            return Err(ParseRadiusError::InvalidRadiusPart(v.to_string()));
        }
        let val_kw = val_part[0];
        let val_f32 = val_part[1].parse();
        if val_f32.is_err() {
            return Err(ParseRadiusError::NotANumber(val_part[1].to_string()));
        }
        let val_f32 = val_f32.unwrap();
        match val_kw {
            "top_left" => top_left = val_f32,
            "top_right" => top_right = val_f32,
            "bottom_left" => bottom_left = val_f32,
            "bottom_right" => bottom_right = val_f32,
            _ => return Err(ParseRadiusError::InvalidRadiusKeyword(val_kw.to_string())),
        }
    }
    return Ok(Radius {
        top_left: top_left,
        top_right: top_right,
        bottom_right: bottom_right,
        bottom_left: bottom_left,
    });
}
