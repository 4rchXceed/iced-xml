//! This module contains all of the utility functions to parse the CSS values into Iced/Rust values.
//!
//!  This is used to parse the styles
use hex_rgb_converter::HexColor;
use iced::{
    Background, Color, Font, Length, Padding, Pixels, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    font::{Family, Stretch, Weight},
    gradient::ColorStop,
    widget::{
        slider::HandleShape,
        text::{LineHeight, Shaping, Wrapping},
        text_input::Side,
    },
};

use crate::{
    rs_utils::to_rad,
    xml_struct::theming::{Fonts, XmlTheme},
};

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
pub fn parse_length(value: &String) -> Length {
    if value.ends_with("fp") {
        if value[..value.len() - 2].parse::<f32>().is_err() {
            println!("Invalid length: {}", value);
            return Length::FillPortion(0);
        }
        return Length::FillPortion(value[..value.len() - 2].parse::<u16>().unwrap());
    } else if value.ends_with("f") {
        if value[..value.len() - 1].parse::<f32>().is_err() {
            println!("Invalid length: {}", value);
            return Length::Fixed(0.0);
        }
        return Length::Fixed(value[..value.len() - 1].parse::<f32>().unwrap());
    } else if value == "max" {
        return Length::Fill;
    } else if value == "min" {
        return Length::Shrink;
    } else {
        println!("Invalid length: {}", value);
        return Length::Fixed(0.0);
    }
}

/// Parse a color value from a string into an Iced Color enum.
/// 4 formats are supported:
/// - Hexadecimal: #RRGGBB or #RGB
/// - RGB: rgb(R, G, B)
/// - RGBA: rgba(R, G, B, A)
/// - Named colors: red, green, blue, etc. (see hex_rgb_converter crate)
pub fn parse_color_op(color: &String) -> Option<Color> {
    if color.trim() == "none" || color.trim().is_empty() {
        return None;
    }
    return Some(parse_color(color));
}

pub fn parse_color(color: &String) -> Color {
    if color.starts_with("#") {
        let color_clean = color.strip_prefix("#");
        if color_clean.is_none() {
            println!("Invalid hex color: {}", color);
            return Color::BLACK;
        }
        let color_clean = color_clean.unwrap();
        let rgb_color = HexColor::new(color_clean).to_rgb();
        return Color::from_rgb(
            (rgb_color.r / 255) as f32,
            (rgb_color.g / 255) as f32,
            (rgb_color.b / 255) as f32,
        );
    } else if color.starts_with("rgba") {
        let color_clean = color.strip_prefix("rgba(");
        if color_clean.is_none() {
            println!("Invalid RGBA color: {}", color);
            return Color::BLACK;
        }
        let color_clean = color_clean.unwrap().strip_suffix(")");
        if color_clean.is_none() {
            println!("Invalid RGBA color: {}", color);
            return Color::BLACK;
        }
        let color_clean = color_clean.unwrap();
        let color_clean = color_clean.split(",").collect::<Vec<&str>>();
        if color_clean.len() != 4 {
            println!("Invalid RGBA color: {}", color);
            return Color::BLACK;
        }
        let color_clean_f32: [f32; 4] = color_clean
            .iter()
            .map(|c| c.trim().parse().unwrap_or(0.0))
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        return Color::from_rgba(
            color_clean_f32[0] / 255.0,
            color_clean_f32[1] / 255.0,
            color_clean_f32[2] / 255.0,
            color_clean_f32[3],
        );
    } else if color.starts_with("rgb") {
        let color_clean = color.strip_prefix("rgb(");
        if color_clean.is_none() {
            println!("Invalid RGB color: {}", color);
            return Color::BLACK;
        }
        let color_clean = color_clean.unwrap().strip_suffix(")");
        if color_clean.is_none() {
            println!("Invalid RGB color: {}", color);
            return Color::BLACK;
        }
        let color_clean = color_clean.unwrap();
        let color_clean = color_clean.split(",").collect::<Vec<&str>>();
        if color_clean.len() != 3 {
            println!("Invalid RGB color: {}", color);
            return Color::BLACK;
        }
        let color_clean_f32: [f32; 3] = color_clean
            .iter()
            .map(|c| c.trim().parse().unwrap_or(0.0))
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        return Color::from_rgb(
            color_clean_f32[0] / 255.0,
            color_clean_f32[1] / 255.0,
            color_clean_f32[2] / 255.0,
        );
    } else {
        let color_by_name = hex_rgb_converter::Color::by_name(color).to_rgb();
        return Color::from_rgb(
            (color_by_name.r / 255) as f32,
            (color_by_name.g / 255) as f32,
            (color_by_name.b / 255) as f32,
        );
    }
}

/// Parse a vector value from a string into an Iced Vector struct.
/// 3 possible errors:
/// - Invalid vector: if the string does not contain exactly 2 values separated by a comma.
/// - Error: if any of the values is not a number (f32).
pub fn parse_vector(value: &String) -> Vector {
    let value_sep = value.split(",").collect::<Vec<&str>>();
    if value_sep.len() != 2 {
        println!("Invalid vector: {}", value);
    }
    let value_f32: Result<[f32; 2], _> = value_sep
        .iter()
        .map(|c| c.parse())
        .map(|c| {
            if c.is_err() {
                println!("Error: {} is not a number (f32)", c.as_ref().unwrap_err());
            }
            c
        })
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect::<Vec<f32>>()
        .try_into();
    if value_f32.is_err() {
        println!("Invalid vector: {}", value);
        return Vector::new(0.0, 0.0);
    }
    let value_f32 = value_f32.unwrap();
    return Vector::new(value_f32[0], value_f32[1]);
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
pub fn parse_radius(value: &String) -> Radius {
    let value_float = value.parse::<f32>();
    if value_float.is_err() {
        let value_sep = value.split(" ").collect::<Vec<&str>>();
        let mut top = 0.0;
        let mut right = 0.0;
        let mut bottom = 0.0;
        let mut left = 0.0;
        for (_, v) in value_sep.iter().enumerate() {
            let val_part = v.split("=").collect::<Vec<&str>>();
            if val_part.len() != 2 {
                println!("Invalid radius part: {}", v);
                continue;
            }
            let val_kw = val_part[0];
            let val_f32 = val_part[1].parse().unwrap();
            match val_kw {
                "top_left" => top = val_f32,
                "top_right" => right = val_f32,
                "bottom_left" => left = val_f32,
                "bottom_right" => bottom = val_f32,
                _ => println!("Invalid radius keyword: {}", val_kw),
            }
        }
        return Radius {
            bottom_left: bottom,
            top_right: top,
            bottom_right: right,
            top_left: left,
        };
    } else {
        let value_float = value_float.unwrap();
        return Radius {
            bottom_left: value_float,
            top_right: value_float,
            bottom_right: value_float,
            top_left: value_float,
        };
    }
}

/// Parse a padding value from a string into an Iced Padding struct.
/// 4 possible errors:
/// - Invalid padding: if the string does not contain exactly 4 values separated by a space.
/// - Error: if any of the values is not a number (f32).
/// - Invalid padding part: if any of the values does not contain a valid keyword (top, right, bottom, left).
/// - Invalid padding keyword: if any of the values does not contain a valid keyword (top, right, bottom, left).
///  The padding can be specified in two ways:
///  - As a single value: `padding: 10;` (all sides will have the same padding)
///  - As a set of values: `padding: top=10 right=20 bottom=30 left=40;` (each side will have its own padding)
pub fn parse_padding(value: &String) -> Padding {
    let value_float = value.parse::<f32>();
    if value_float.is_err() {
        let value_sep = value.split(" ").collect::<Vec<&str>>();
        let mut top = 0.0;
        let mut right = 0.0;
        let mut bottom = 0.0;
        let mut left = 0.0;
        for (_, v) in value_sep.iter().enumerate() {
            let val_part = v.split("=").collect::<Vec<&str>>();
            if val_part.len() != 2 {
                println!("Invalid padding part: {}", v);
                continue;
            }
            let val_kw = val_part[0];
            let val_f32 = val_part[1].parse().unwrap();
            match val_kw {
                "top" => top = val_f32,
                "right" => right = val_f32,
                "bottom" => bottom = val_f32,
                "left" => left = val_f32,
                _ => println!("Invalid radius keyword: {}", val_kw),
            }
        }
        return Padding {
            top: top,
            right: right,
            left: left,
            bottom: bottom,
        };
    } else {
        let value_float = value_float.unwrap();
        return Padding::new(value_float);
    }
}

/// Parse a horizontal alignment value from a string into an Iced Horizontal enum.
/// 3 possible values: start, center, end. If the value is not one of these, it will default to start.
pub fn parse_align_x(value: &String) -> Horizontal {
    match value.as_str() {
        "start" => Horizontal::Left,
        "center" => Horizontal::Center,
        "end" => Horizontal::Right,
        _ => Horizontal::Left,
    }
}

/// Parse a vertical alignment value from a string into an Iced Vertical enum.
/// 3 possible values: start, center, end. If the value is not one of these, it will default to start.
pub fn parse_align_y(value: &String) -> Vertical {
    match value.as_str() {
        "start" => Vertical::Top,
        "center" => Vertical::Center,
        "end" => Vertical::Bottom,
        _ => Vertical::Top,
    }
}

/// Parse a f32 value from a string. If the value is not a valid f32, it will default to 0.0.
pub fn parse_value(value: &String) -> f32 {
    value.parse().unwrap_or_default()
}

/// Parse a i32 value from a string. If the value is not a valid i32, it will default to 0.
pub fn parse_value_int(value: &String) -> i32 {
    value.parse().unwrap_or_default()
}

/// Parse a f32 value from a string. If the value is not a valid f32, it will return None.
pub fn parse_value_maybe(value: &String) -> Option<f32> {
    value.parse().ok()
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
pub fn parse_font(value: &String, fonts: &Fonts) -> Font {
    let mut family = Family::Serif;
    let mut weight = Weight::Normal;
    let mut stretch = Stretch::Normal;
    let mut style = iced::font::Style::Normal;
    let parts = value.split(' ').collect::<Vec<&str>>();
    for part in parts {
        let parts: Vec<&str> = part.split("=").collect();
        if parts.len() != 2 {
            println!("Part: {:?} MUST be in the format 'key=value'", parts);
            continue;
        }
        let key = parts[0];
        let value = parts[1];
        match key {
            "family" => {
                parse_font_family(&mut family, value, fonts);
            }
            "weight" => {
                parse_font_weight(&mut weight, value);
            }
            "stretch" => {
                parse_font_stretch(&mut stretch, value);
            }
            "style" => {
                parse_font_style(&mut style, value);
            }
            _ => println!("Invalid font property: {}", key),
        }
    }
    Font {
        family: family,
        weight: weight,
        stretch: stretch,
        style: style,
    }
}

/// Parse a font style value from a string into an Iced FontStyle enum.
/// 3 possible values: normal, italic, oblique. If the value is not one of these, it will default to normal.
pub fn parse_font_style(style: &mut iced::font::Style, value: &str) {
    *style = match value {
        "normal" => iced::font::Style::Normal,
        "italic" => iced::font::Style::Italic,
        "oblique" => iced::font::Style::Oblique,
        _ => iced::font::Style::Normal,
    }
}

/// Parse a font stretch value from a string into an Iced FontStretch enum.
/// Possible values: normal, condensed, expanded, extra-condensed, extra-expanded, semi-condensed, semi-expanded, ultra-condensed, ultra-expanded. If the value is not one of these, it will default to normal.
pub fn parse_font_stretch(stretch: &mut Stretch, value: &str) {
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
        _ => Stretch::Normal,
    }
}

/// Parse a font weight value from a string into an Iced FontWeight enum.
/// Possible values: normal, bold, bolder, lighter, 100, 200, 300, 400, 500, 600, 700, 800, 900. If the value is not one of these, it will default to normal.
pub fn parse_font_weight(weight: &mut Weight, value: &str) {
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
        _ => Weight::Normal,
    }
}

/// Parse a font family value from a string into an Iced FontFamily enum.
/// Possible values: serif, sans-serif, monospace, cursive, fantasy, or a custom font name (must be registered in the Fonts struct). If the value is not one of these, it will default to serif.
pub fn parse_font_family(family: &mut Family, value: &str, fonts: &Fonts) {
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
                println!("Invalid font family: {}, using serif as default", value);
                Family::Serif
            }
        }
    }
}

/// Parse a shaping value from a string into an Iced Shaping enum.
/// Possible values: quality, performance, auto. If the value is not one of these, it will default to auto.
pub fn parse_shaping(value: &str) -> Shaping {
    match value {
        "quality" => Shaping::Advanced,
        "performance" => Shaping::Basic,
        "auto" => Shaping::Auto,
        _ => Shaping::Auto,
    }
}

/// Parse the text wrapping value from a string into an Iced Wrapping enum.
/// Possible values: word, glyph, word-or-glyph, none. If the value is not one of these, it will default to none.
pub fn parse_text_wrapping(value: &str) -> Wrapping {
    match value {
        "word" => Wrapping::Word,
        "glyph" => Wrapping::Glyph,
        "word-or-glyph" => Wrapping::WordOrGlyph,
        "none" => Wrapping::None,
        _ => Wrapping::None,
    }
}

/// Parse the checkbox icon value from a string into an Iced Checkbox Icon struct.
/// The icon value can be specified in the following format:
/// "<icon> <size> <line-height-type> <line-height-value>"
/// The icon is a single character (e.g. "☑") and must be enclosed in double quotes. The size is a number (e.g. 20) and is optional.
/// The line-height-type can be either "absolute" or "relative" and is required. The line-height-value is a number (e.g. 15) and is required.
pub fn parse_checkbox_icon(
    value: &str,
    font: &Font,
    shaping: Shaping,
) -> Option<iced::widget::checkbox::Icon<Font>> {
    if value == "none" {
        return None;
    }
    let err_msg = "Invalid icon: must be (example) `checkbox-icon: \"☑\" 20 absolute 15; (font settings are taken from the checkbox)`";
    if value.is_empty() {
        println!("{}", err_msg);
        return None;
    }
    let parts: Vec<&str> = value.split(" ").collect();
    if parts.len() != 4 && parts.len() != 3 {
        println!("{}", err_msg);
        return None;
    }
    let mut chars = parts[0].chars();
    // let first = char.next();
    if chars.next() != Some('"') {
        println!("{}", err_msg);
        return None;
    }
    let char = chars.next().unwrap_or('☑');
    if chars.next() != Some('"') {
        println!("{}", err_msg);
        return None;
    }
    let mut i = 1;
    let size_str = parts[1].parse::<f32>();
    let mut size: Option<Pixels> = None;
    if size_str.is_ok() {
        size = Some(Pixels(size_str.unwrap()));
        i = 2;
    }
    let line_height = match parts[i] {
        "absolute" => LineHeight::Absolute(Pixels(parts[i + 1].parse::<f32>().unwrap_or(10.0))),
        "relative" => LineHeight::Relative(parts[i + 1].parse::<f32>().unwrap_or(10.0)),
        _ => LineHeight::Relative(10.0),
    };
    return Some(iced::widget::checkbox::Icon {
        font: font.clone(),
        code_point: char,
        size: size,
        line_height: line_height,
        shaping: shaping,
    });
}

/// Parse the select icon value from a string into an Iced Select Icon struct.
/// The icon value can be specified in the following format:
/// "<icon> <size> <side> <spacing>"
/// The icon is a single character (e.g. ">") and must be enclosed in double quotes.
/// The size is a number (e.g. 20) and is optional.
/// The side can be either "left" or "right" and is required.
/// The spacing is a number (e.g. 15) and is required.
pub fn parse_select_icon(value: &str, font: &Font) -> Option<iced::widget::text_input::Icon<Font>> {
    if value == "none" {
        return None;
    }
    let err_msg = "Invalid icon: must be (example) `select-icon: \">\" 20 left 15; (font settings are taken from the checkbox)`";
    if value.is_empty() {
        println!("{}", err_msg);
        return None;
    }
    let parts: Vec<&str> = value.split(" ").collect();
    if parts.len() != 4 && parts.len() != 3 {
        println!("{}", err_msg);
        return None;
    }
    let mut chars = parts[0].chars();
    // let first = char.next();
    if chars.next() != Some('"') {
        println!("{}", err_msg);
        return None;
    }
    let char = chars.next().unwrap_or('☑');
    if chars.next() != Some('"') {
        println!("{}", err_msg);
        return None;
    }
    let mut i = 1;
    let size_str = parts[1].parse::<f32>();
    let mut size: Option<Pixels> = None;
    if size_str.is_ok() {
        size = Some(Pixels(size_str.unwrap()));
        i = 2;
    }
    let side = match parts[i] {
        "left" => Side::Left,
        "right" => Side::Right,
        _ => {
            println!(
                "Unknown side: {}, expected `left` or `right`. Using left as default",
                parts[i]
            );
            Side::Left
        }
    };
    let spacing_op = parts[i + 1].parse::<f32>();
    if spacing_op.is_err() {
        println!("Error: spacing {} is not a number (f32)", parts[i + 1]);
        return None;
    }
    return Some(iced::widget::text_input::Icon {
        font: font.clone(),
        code_point: char,
        size: size,
        side: side,
        spacing: spacing_op.unwrap(),
    });
}

/// Parse the line height value from a string into an Iced LineHeight enum.
/// The line height value can be specified in the following format:
/// "absolute <nbr>" or "relative <nbr>"
pub fn parse_line_height(value: &str) -> LineHeight {
    let split = value.split(" ").collect::<Vec<&str>>();
    if split.len() != 2 {
        println!("Line height must be specified as `absolute <nbr>` or `relative <nbr>`");
        return LineHeight::Relative(10.0);
    }
    return match split[0] {
        "absolute" => LineHeight::Absolute(Pixels(split[1].parse::<f32>().unwrap_or(10.0))),
        "relative" => LineHeight::Relative(split[1].parse::<f32>().unwrap_or(10.0)),
        _ => LineHeight::Relative(10.0),
    };
}

/// Parse the pane axis value from a string into an Iced PaneGrid Axis enum.
/// The pane axis value can be specified in the following format:
/// "horizontal" or "vertical"
pub fn parse_pane_axis(value: &str) -> iced::widget::pane_grid::Axis {
    match value {
        "horizontal" => iced::widget::pane_grid::Axis::Horizontal,
        "vertical" => iced::widget::pane_grid::Axis::Vertical,
        _ => {
            println!(
                "Invalid axis: {}, expected `horizontal` or `vertical`. Using horizontal as default",
                value
            );
            iced::widget::pane_grid::Axis::Horizontal
        }
    }
}

/// Parse the anchor value from a string into an Iced Anchor enum.
/// The anchor value can be specified in the following format:
/// "top", "bottom", "left", "right"
pub fn check_anchor(value: &str) -> String {
    match value {
        "top" | "bottom" | "left" | "right" => value.to_string(),
        _ => {
            println!(
                "Invalid anchor: {}, expected `top`, `bottom`, `left` or `right`. Using top as default",
                value
            );
            "top".to_string()
        }
    }
}

/// Parse the slider handle theme value from a string into an Iced HandleShape enum.
/// The slider handle theme value can be specified in the following format:
/// "circle <radius>" or "rectangle <width> <border radius>"
pub fn parse_slider_handle_theme(value: &str, theme: &XmlTheme) -> HandleShape {
    let msg = "Slider handle theme must be specified as `circle <radius>` or `rectangle <width> <border radius>`";
    let split = value.split(" ").collect::<Vec<&str>>();
    if split.len() < 1 {
        println!("{}", msg);
        return HandleShape::Circle { radius: 10.0 };
    }
    if split[0] == "circle" {
        if split.len() != 2 {
            println!("{}", msg);
        } else {
            let radius = split[1].parse::<f32>();
            if radius.is_err() {
                println!("Invalid radius: {}, must be a number", split[1]);
                return HandleShape::Circle { radius: 10.0 };
            }
            return HandleShape::Circle {
                radius: radius.unwrap(),
            };
        }
    } else if split[0] == "rectangle" {
        if split.len() != 3 {
            println!("{}", msg);
        } else {
            let width = split[1].parse::<u16>();
            let border_radius = split[2].parse::<f32>();
            if width.is_err() || border_radius.is_err() {
                println!(
                    "Invalid width or border radius: {}, {}, must be numbers",
                    split[1], split[2]
                );
                return HandleShape::Circle { radius: 10.0 };
            }
            return HandleShape::Rectangle {
                width: width.unwrap(),
                border_radius: theme.border_radius,
            };
        }
    } else {
        println!("{}", msg);
    }
    return HandleShape::Circle { radius: 10.0 };
}

/// Parse two f32 values from a string into a tuple of two f32 values.
/// The two f32 values can be specified in the following format:
/// "<value1> <value2>" or "<value>"
pub fn parse_two_f32(value: &str) -> (f32, f32) {
    let msg = format!(
        "Invalid value: {}, must be two numbers separated by a space or one number",
        value
    );
    let value_f32 = value.parse::<f32>();
    if value_f32.is_ok() {
        return (value_f32.clone().unwrap(), value_f32.unwrap());
    }
    let split = value.split(" ").collect::<Vec<&str>>();
    if split.len() != 2 {
        println!("{}", msg);
        return (0.0, 0.0);
    }
    let first = split[0].parse::<f32>();
    let second = split[1].parse::<f32>();
    if first.is_err() || second.is_err() {
        println!("{}", msg);
        return (0.0, 0.0);
    }
    return (first.unwrap(), second.unwrap());
}

/// Parse the center type value from a string into a boolean.
/// The center type value can be specified in the following format:
/// "align" or "center". If the value is "align", it will return true. If the value is "center", it will return false.
/// If the value is not one of these, it will default to false and print an error message.
pub fn parse_center_type(value: &str) -> bool {
    return match value {
        "align" => true,
        "center" => false,
        _ => {
            println!(
                "Invalid center type: {}, expected align or center. Using center as default",
                value
            );
            false
        }
    };
}

/// Parse a background value from a string into an Iced Background enum.
/// The background value can be specified in the following format:
/// "linear-gradient(<angle>deg, <color1> <position1>%, <color2> <position2>%, ...)" or "<color>". The linear gradient can have a maximum of 8 colors.
/// Or it can be a single color value in any of the formats supported by the parse_color function.
/// If the value is not in the correct format, it will default to a transparent color and print an error message.
pub fn parse_background(value: &str) -> Background {
    let err_msg = format!(
        "Linear gradiant must be in this format: linear-gradient(Ndeg, <color>,...) (max 8 colors). Currently: {}",
        value
    );
    let fallback = Background::Color(Color::TRANSPARENT);
    let value = value.trim();
    if value.starts_with("linear-gradient(") && value.ends_with(")") {
        let value = value
            .strip_prefix("linear-gradient(")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let parts: Vec<&str> = value.split("deg").collect();
        if parts.len() > 0 {
            let rotation_unparsed = parts[0].trim();
            let rotation = rotation_unparsed.parse::<f32>();
            if rotation.is_err() {
                println!("{}", err_msg);
                return fallback;
            }
            let rotation_rad = to_rad(rotation.unwrap());
            let value = value.split_once("deg").unwrap().1.trim().strip_prefix(",");
            if value.is_none() {
                println!("{}", err_msg);
                return fallback;
            }
            let gradiant_stops: Vec<&str> = value
                .unwrap()
                .split("%")
                .map(|s| s.trim().strip_prefix(",").unwrap_or(s).trim_start())
                .filter(|s| !s.is_empty())
                .collect();
            let mut colors: [Option<ColorStop>; 8] = [None; 8];
            if gradiant_stops.len() > 8 {
                println!("{}", err_msg);
                return fallback;
            }
            for (i, grandiant_stop) in gradiant_stops.iter().enumerate() {
                let grandiant_stop_parts = grandiant_stop.trim().rsplit_once(char::is_whitespace); //FIX
                if grandiant_stop_parts.is_none() {
                    println!("{}", err_msg);
                    return fallback;
                }
                let grandiant_stop_parts = grandiant_stop_parts.unwrap();
                let color = parse_color(&String::from(grandiant_stop_parts.0));

                let position = grandiant_stop_parts.1.parse::<f32>();
                if position.is_err() {
                    println!("{}", err_msg);
                    return fallback;
                }
                colors[i] = Some(ColorStop {
                    color: color,
                    offset: position.unwrap() / 100.0,
                });
            }
            return Background::Gradient(iced::Gradient::Linear(iced::gradient::Linear {
                angle: iced::Radians(rotation_rad),
                stops: colors,
            }));
        } else {
            println!("{}", err_msg);
            return fallback;
        }
    } else {
        return Background::Color(parse_color(&String::from(value)));
    }
}

/// Parse a boolean value from a string into a boolean.
/// The boolean value can be specified in the following format:
/// "true" or "false". If the value is not one of these, it will default to false and print an error message.
pub fn parse_bool(value: &str) -> bool {
    return match value {
        "true" => true,
        "false" => false,
        _ => {
            println!(
                "Invalid boolean value: {}, expected true or false. Using false as default",
                value
            );
            false
        }
    };
}

/// Parse a text alignment value from a string into an Iced Text Alignment enum.
/// The text alignment value can be specified in the following format:
/// "left", "center", "right", "justified", "default". If the value is not one of these, it will default to default and print an error message.
pub fn parse_text_alignment(value: &str) -> iced::widget::text::Alignment {
    match value {
        "left" => iced::widget::text::Alignment::Center,
        "center" => iced::widget::text::Alignment::Center,
        "right" => iced::widget::text::Alignment::Right,
        "justified" => iced::widget::text::Alignment::Justified,
        "default" => iced::widget::text::Alignment::Default,
        _ => {
            println!(
                "Invalid text alignment: {}, expected left, center, right, justified or default. Using default as",
                value
            );
            iced::widget::text::Alignment::Default
        }
    }
}

/// Parse a time value from a string into a f32 value in seconds.
/// The time value can be specified in the following format:
/// "<value>ms" or "<value>s". If the value is not in the correct format, it will default to 0.0 and print an error message.
pub fn parse_time(value: &str) -> f32 {
    if value.ends_with("ms") {
        let value = value.strip_suffix("ms").unwrap();
        return parse_value(&String::from(value)) / 1000.0;
    }
    return parse_value(&String::from(value));
}
