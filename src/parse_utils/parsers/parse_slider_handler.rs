use iced::widget::slider::HandleShape;
use thiserror::Error;

use crate::xml_struct::theming::XmlTheme;

#[derive(Debug, Error)]
pub enum ParseSliderHandleThemeError {
    #[error(
        "Invalid slider handle theme: {0}. Slider handle theme must be specified as `circle <radius>` or `rectangle <width>`"
    )]
    InvalidSliderHandleTheme(String),
    #[error("Invalid circle: {0}. Circle radius must be a number. (example: `circle 10.0`)")]
    InvalidCircle(String),
    #[error("Invalid rectangle: {0}. Rectangle width must be a number. (example: `rectangle 10`)")]
    InvalidRectangle(String),
    #[error("Invalid number: {0}. Must be a number.")]
    NotANumber(String),
}

/// Parse the slider handle theme value from a string into an Iced HandleShape enum.
/// The slider handle theme value can be specified in the following format:
/// "circle <radius>" or "rectangle <width> <border radius>"
pub fn parse_slider_handle_theme(
    value: &str,
    theme: &XmlTheme,
) -> Result<HandleShape, ParseSliderHandleThemeError> {
    let split = value.split(" ").collect::<Vec<&str>>();
    if split.len() < 1 {
        return Err(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
            value.to_string(),
        ));
    }

    let first = *split
        .get(0)
        .ok_or(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
            value.to_string(),
        ))?;
    let second = *split
        .get(1)
        .ok_or(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
            value.to_string(),
        ))?;

    if first == "circle" {
        if split.len() != 2 {
            return Err(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
                value.to_string(),
            ));
        } else {
            let radius = second
                .parse::<f32>()
                .map_err(|_| ParseSliderHandleThemeError::NotANumber(second.to_string()))?;

            return Ok(HandleShape::Circle { radius: radius });
        }
    } else if first == "rectangle" {
        if split.len() != 2 {
            return Err(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
                value.to_string(),
            ));
        } else {
            let width = second
                .parse::<u16>()
                .map_err(|_| ParseSliderHandleThemeError::InvalidRectangle(second.to_string()))?;

            return Ok(HandleShape::Rectangle {
                width: width,
                border_radius: theme.border_radius,
            });
        }
    } else {
        return Err(ParseSliderHandleThemeError::InvalidSliderHandleTheme(
            value.to_string(),
        ));
    }
}
