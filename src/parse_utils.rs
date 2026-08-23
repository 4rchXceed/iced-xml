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

pub fn parse_vector(value: &String) -> Vector {
    let value_sep = value.split(",").collect::<Vec<&str>>();
    if value_sep.len() != 2 {
        println!("Invalid vector: {}", value);
    }
    let value_f32: [f32; 2] = value_sep
        .iter()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap();
    return Vector::new(value_f32[0], value_f32[1]);
}

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

pub fn parse_align_x(value: &String) -> Horizontal {
    match value.as_str() {
        "start" => Horizontal::Left,
        "center" => Horizontal::Center,
        "end" => Horizontal::Right,
        _ => Horizontal::Left,
    }
}

pub fn parse_align_y(value: &String) -> Vertical {
    match value.as_str() {
        "start" => Vertical::Top,
        "center" => Vertical::Center,
        "end" => Vertical::Bottom,
        _ => Vertical::Top,
    }
}

pub fn parse_value(value: &String) -> f32 {
    value.parse().unwrap_or_default()
}

pub fn parse_value_int(value: &String) -> i32 {
    value.parse().unwrap_or_default()
}

pub fn parse_value_maybe(value: &String) -> Option<f32> {
    value.parse().ok()
}

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

pub fn parse_font_style(style: &mut iced::font::Style, value: &str) {
    *style = match value {
        "normal" => iced::font::Style::Normal,
        "italic" => iced::font::Style::Italic,
        "oblique" => iced::font::Style::Oblique,
        _ => iced::font::Style::Normal,
    }
}

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

pub fn parse_font_family(family: &mut Family, value: &str, fonts: &Fonts) {
    *family = match value {
        "serif" => Family::Serif,
        "fantasy" => Family::Fantasy,
        "cursive" => Family::Cursive,
        "monospace" => Family::Monospace,
        "sans-serif" => Family::SansSerif,
        _ => {
            let font = fonts.get(&String::from(value));
            if font.is_some() {
                Family::Name(font.unwrap())
            } else {
                println!("Invalid font family: {}, using serif as default", value);
                Family::Serif
            }
        }
    }
}

pub fn parse_shaping(value: &str) -> Shaping {
    match value {
        "quality" => Shaping::Advanced,
        "performance" => Shaping::Basic,
        "auto" => Shaping::Auto,
        _ => Shaping::Auto,
    }
}

pub fn parse_text_wrapping(value: &str) -> Wrapping {
    match value {
        "word" => Wrapping::Word,
        "glyph" => Wrapping::Glyph,
        "word-or-glyph" => Wrapping::WordOrGlyph,
        "none" => Wrapping::None,
        _ => Wrapping::None,
    }
}

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

pub fn parse_time(value: &str) -> f32 {
    if value.ends_with("ms") {
        let value = value.strip_suffix("ms").unwrap();
        return parse_value(&String::from(value)) / 1000.0;
    }
    return parse_value(&String::from(value));
}
