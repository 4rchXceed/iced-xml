use iced::{
    Font,
    font::{Family, Stretch, Weight},
};
use thiserror::Error;

use crate::xml_struct::theming::Fonts;

#[derive(Debug, Error)]
pub enum ParseFontError {
    #[error("Invalid font: {0}")]
    InvalidFont(String),
    #[error("Invalid font property: {0}")]
    InvalidFontProperty(String),
    #[error("Invalid font family: {0}")]
    InvalidFontFamily(String),
    #[error("Invalid font weight: {0}")]
    InvalidFontWeight(String),
    #[error("Invalid font stretch: {0}")]
    InvalidFontStretch(String),
    #[error("Invalid font style: {0}")]
    InvalidFontStyle(String),
}

/// Parse a font value from a string into an Iced Font struct.
/// The font value can be specified in the following format:
/// family=<font family> weight=<font weight> stretch=<font stretch> style=<font style>
/// The font family can be one of the following: serif, sans-serif, monospace, cursive, fantasy, or a custom font name (must be registered in the Fonts struct).
/// The font weight can be one of the following: normal, bold, bolder, lighter, 100, 200, 300, 400, 500, 600, 700, 800, 900.
/// The font stretch can be one of the following: normal, condensed, expanded, extra-condensed, extra-expanded, semi-condensed, semi-expanded, ultra-condensed, ultra-expanded.
/// The font style can be one of the following: normal, italic, oblique.
/// If any of the values are not specified, they will default to the following:
/// - family: serif
/// - weight: normal
/// - stretch: normal
/// - style: normal
/// - other: (defined in the Engine settings)
pub fn parse_font(value: &String, fonts: &Fonts) -> Result<Font, ParseFontError> {
    let mut family = Family::Serif;
    let mut weight = Weight::Normal;
    let mut stretch = Stretch::Normal;

    let mut style = iced::font::Style::Normal;

    let parts = value.split(' ').collect::<Vec<&str>>();

    for part in parts {
        let parts: Vec<&str> = part.split("=").collect();
        if parts.len() != 2 {
            return Err(ParseFontError::InvalidFontProperty(part.to_string()));
        }
        let key = parts[0];
        let value = parts[1];
        match key {
            "family" => {
                parse_font_family(&mut family, value, fonts)?;
            }
            "weight" => {
                parse_font_weight(&mut weight, value)?;
            }
            "stretch" => {
                parse_font_stretch(&mut stretch, value)?;
            }
            "style" => {
                parse_font_style(&mut style, value)?;
            }
            _ => return Err(ParseFontError::InvalidFontProperty(key.to_string())),
        }
    }
    return Ok(Font {
        family: family,
        weight: weight,
        stretch: stretch,
        style: style,
    });
}

/// Parse a font style value from a string into an Iced FontStyle enum.
/// 3 possible values: normal, italic, oblique. If the value is not one of these, it will default to normal.
pub fn parse_font_style(style: &mut iced::font::Style, value: &str) -> Result<(), ParseFontError> {
    *style = match value {
        "normal" => iced::font::Style::Normal,
        "italic" => iced::font::Style::Italic,
        "oblique" => iced::font::Style::Oblique,
        _ => {
            return Err(ParseFontError::InvalidFontStyle(value.to_string()));
        }
    };

    return Ok(());
}

/// Parse a font stretch value from a string into an Iced FontStretch enum.
/// Possible values: normal, condensed, expanded, extra-condensed, extra-expanded, semi-condensed, semi-expanded, ultra-condensed, ultra-expanded. If the value is not one of these, it will default to normal.
pub fn parse_font_stretch(stretch: &mut Stretch, value: &str) -> Result<(), ParseFontError> {
    *stretch = match value {
        "normal" => Stretch::Normal,
        "condensed" => Stretch::Condensed,
        "expanded" => Stretch::Expanded,
        "extra-condensed" => Stretch::ExtraCondensed,
        "extra-expanded" => Stretch::ExtraExpanded,
        "semi-condensed" => Stretch::SemiCondensed,
        "semi-expanded" => Stretch::SemiExpanded,
        "ultra-condensed" => Stretch::UltraCondensed,
        "ultra-expanded" => Stretch::UltraExpanded,
        _ => {
            return Err(ParseFontError::InvalidFontStretch(value.to_string()));
        }
    };

    return Ok(());
}

/// Parse a font weight value from a string into an Iced FontWeight enum.
/// Possible values: normal, bold, bolder, lighter, 100, 200, 300, 400, 500, 600, 700, 800, 900. If the value is not one of these, it will default to normal.
pub fn parse_font_weight(weight: &mut Weight, value: &str) -> Result<(), ParseFontError> {
    *weight = match value {
        "normal" => Weight::Normal,
        "bold" => Weight::Bold,
        "black" => Weight::Black,
        "extra-bold" => Weight::ExtraBold,
        "extra-light" => Weight::ExtraLight,
        "light" => Weight::Light,
        "medium" => Weight::Medium,
        "semibold" => Weight::Semibold,
        "thin" => Weight::Thin,
        _ => {
            return Err(ParseFontError::InvalidFontWeight(value.to_string()));
        }
    };

    return Ok(());
}

/// Parse a font family value from a string into an Iced FontFamily enum.
/// Possible values: serif, sans-serif, monospace, cursive, fantasy, or a custom font name (must be registered in the Fonts struct). If the value is not one of these, it will default to serif.
pub fn parse_font_family(
    family: &mut Family,
    value: &str,
    fonts: &Fonts,
) -> Result<(), ParseFontError> {
    *family = match value {
        "serif" => Family::Serif,
        "fantasy" => Family::Fantasy,
        "cursive" => Family::Cursive,
        "monospace" => Family::Monospace,
        "sans-serif" => Family::SansSerif,
        _ => {
            let font = fonts.iter().find(|f| f.0 == String::from(value));
            if font.is_some() {
                Family::Name(font.unwrap().1)
            } else {
                return Err(ParseFontError::InvalidFontFamily(value.to_string()));
            }
        }
    };

    return Ok(());
}
