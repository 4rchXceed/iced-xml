use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseTwoNumbersError {
    #[error("Invalid value: {0}, must be two numbers separated by a space or one number")]
    InvalidValue(String),
}

/// Parse two f32 values from a string into a tuple of two f32 values.
/// The two f32 values can be specified in the following format:
/// "<value1> <value2>" or "<value>"
pub fn parse_two_f32(value: &str) -> Result<(f32, f32), ParseTwoNumbersError> {
    let value_f32 = value.parse::<f32>();

    if let Ok(value) = value_f32 {
        return Ok((value, value));
    }

    let split = value.split(" ").collect::<Vec<&str>>();

    let first = *split
        .get(0)
        .ok_or(ParseTwoNumbersError::InvalidValue(value.to_string()))?;
    let second = *split
        .get(1)
        .ok_or(ParseTwoNumbersError::InvalidValue(value.to_string()))?;

    let first = first
        .parse::<f32>()
        .map_err(|_| ParseTwoNumbersError::InvalidValue(value.to_string()))?;
    let second = second
        .parse::<f32>()
        .map_err(|_| ParseTwoNumbersError::InvalidValue(value.to_string()))?;

    return Ok((first, second));
}
