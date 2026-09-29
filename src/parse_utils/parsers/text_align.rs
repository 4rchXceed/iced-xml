use iced::widget::text::Alignment;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseTextAlignmentError {
    #[error("Invalid text alignment: {0}, expected left, center, right, justified or default.")]
    InvaidTextAlignment(String),
}

/// Parse a text alignment value from a string into an Iced Text Alignment enum.
/// The text alignment value can be specified in the following format:
/// "left", "center", "right", "justified", "default". If the value is not one of these, it will default to default and print an error message.
pub fn parse_text_alignment(value: &str) -> Result<Alignment, ParseTextAlignmentError> {
    match value {
        "left" => Ok(Alignment::Left),
        "center" => Ok(Alignment::Center),
        "right" => Ok(Alignment::Right),
        "justified" => Ok(Alignment::Justified),
        "default" => Ok(Alignment::Default),
        _ => {
            return Err(ParseTextAlignmentError::InvaidTextAlignment(
                value.to_string(),
            ));
        }
    }
}
