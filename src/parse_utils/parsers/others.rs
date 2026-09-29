use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseBoolError {
    #[error("Invalid boolean: {0}. Expected true or false")]
    InvalidTrueFalse(String),
}

/// Parse a boolean value from a string into a boolean.
/// The boolean value can be specified in the following format:
/// "true" or "false". If the value is not one of these, it will default to false and print an error message.
pub fn parse_bool(value: &str) -> Result<bool, ParseBoolError> {
    return match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => return Err(ParseBoolError::InvalidTrueFalse(value.to_string())),
    };
}

/// Parse a time value from a string into a f32 value in seconds.
/// The time value can be specified in the following format:
/// "<value>ms" or "<value>s". If the value is not in the correct format, it will default to 0.0 and print an error message.
pub fn parse_time(value: &str) -> Result<f32, String> {
    if value.ends_with("ms") {
        let value = value.strip_suffix("ms").unwrap();
        return Ok(parse_value(&String::from(value))? / 1000.0);
    }
    if value.ends_with("s") {
        let value = value.strip_suffix("s").unwrap();
        return Ok(parse_value(&String::from(value))?);
    }
    return Err(format!("Invalid number with suffix: {value}"));
}

/// Parse a f32 value from a string. If the value is not a valid f32, it will default to 0.0.
pub fn parse_value(value: &String) -> Result<f32, String> {
    value.parse().map_err(|_| format!("Invalid int: {value}"))
}

/// Parse a i32 value from a string. If the value is not a valid i32, it will default to 0.
pub fn parse_value_int(value: &String) -> Result<i32, String> {
    value.parse().map_err(|_| format!("Invalid int: {value}"))
}

/// Parse a f32 value from a string. If the value is not a valid f32, it will return None.
pub fn parse_value_maybe(value: &String) -> Option<f32> {
    value.parse().ok()
}
