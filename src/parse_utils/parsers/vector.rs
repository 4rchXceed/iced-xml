use iced::Vector;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ParseVectorError {
    #[error("Invalid vector: {0}")]
    InvalidVector(String),
    #[error("Error: {0} is not a number (f32)")]
    NotANumber(String),
}

/// Parse a vector value from a string into an Iced Vector struct.
/// 3 possible errors:
/// - Invalid vector: if the string does not contain exactly 2 values separated by a comma.
/// - Error: if any of the values is not a number (f32).
pub fn parse_vector(value: &String) -> Result<Vector, ParseVectorError> {
    let value_sep = value.split(",").collect::<Vec<&str>>();

    let value_f32 = value_sep
        .iter()
        .map(|c| {
            c.parse::<f32>()
                .map_err(|_| ParseVectorError::NotANumber(c.to_string()))
        })
        .collect::<Vec<Result<f32, ParseVectorError>>>();

    let val_01 = value_f32[0].as_ref().map_err(|e| e.clone())?;
    let val_02 = value_f32[1].as_ref().map_err(|e| e.clone())?;

    if value_sep.len() != 2 {
        return Err(ParseVectorError::InvalidVector(value.clone()));
    }

    return Ok(Vector::new(*val_01, *val_02));
}
