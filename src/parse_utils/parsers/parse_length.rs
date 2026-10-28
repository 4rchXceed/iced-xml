use iced::Length;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LengthError {
    #[error("Invalid fill length: {0}")]
    InvalidFillPortionLength(String),
    #[error("Invalid fixed length: {0}")]
    InvalidFixedLength(String),
    #[error("Invalid length type (must be Nfp, Nf, max or min): {0}")]
    InvalidLengthType(String),
}

/// Parse a length value from a string into an Iced Length enum.
///
///  There are 4 units:
/// - Nfp => (from iced doc:) Fills a portion of the remaining space relative to other elements.
/// Let’s say we have two elements: one with FillPortion(2) and one with FillPortion(3). The first will get 2 portions of the available space, while the second one would get 3.
/// Length::Fill is equivalent to Length::FillPortion(1). See iced's Length::FillPortion documentation for more details.
/// - Nf => Fixed length in pixels. For example, 100f is 100 pixels.
/// - min => Use the minimum space possible, while trying to keep the content visible. This is equivalent to Length::Shrink.
/// - max => Use the maximum space possible. This is equivalent to Length::Fill
///
/// Else it returns a fixed length of 0.0. And prints an error message to the console.
pub fn parse_length(value: &String) -> Result<Length, LengthError> {
    if value.ends_with("fp") {
        let value = value
            .strip_suffix("fp")
            .ok_or(LengthError::InvalidFillPortionLength(value.clone()))?
            .parse::<u16>()
            .map_err(|_| LengthError::InvalidFillPortionLength(value.clone()))?;

        return Ok(Length::FillPortion(value));
    } else if value.ends_with("f") {
        let value = value
            .strip_suffix("f")
            .ok_or(LengthError::InvalidFixedLength(value.clone()))?
            .parse::<f32>()
            .map_err(|_| LengthError::InvalidFixedLength(value.clone()))?;

        return Ok(Length::Fixed(value));
    } else if value == "max" {
        return Ok(Length::Fill);
    } else if value == "min" {
        return Ok(Length::Shrink);
    } else {
        return Err(LengthError::InvalidLengthType(value.clone()));
    }
}
