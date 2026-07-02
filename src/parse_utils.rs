use hex_rgb_converter::HexColor;
use iced::{
    Color, Font, Length, Padding, Pixels, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    font::{Family, Stretch, Weight},
    widget::{
        checkbox::Icon,
        text::{LineHeight, Shaping, Wrapping},
    },
};

pub fn safe_read_file(filename: &str) -> String {
    let content = std::fs::read_to_string(filename);
    if content.is_err() {
        println!("Failed to read main window file: {}", filename);
    }
    return content.unwrap();
}

pub fn parse_length(value: &String) -> Length {
    if value.ends_with("fp") {
        return Length::FillPortion(value[..value.len() - 2].parse().unwrap());
    } else if value.ends_with("f") {
        return Length::Fixed(value[..value.len() - 1].parse().unwrap());
    } else if value == "max" {
        return Length::Fill;
    } else if value == "min" {
        return Length::Shrink;
    } else {
        println!("Invalid length: {}", value);
        return Length::Fixed(0.0);
    }
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
            .map(|c| c.parse().unwrap())
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        return Color::from_rgba(
            color_clean_f32[0] / 255.0,
            color_clean_f32[1] / 255.0,
            color_clean_f32[2] / 255.0,
            color_clean_f32[3] / 255.0,
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
            .map(|c| c.trim().parse().unwrap())
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
                "left" => left = val_f32,
                "bottom" => bottom = val_f32,
                _ => println!("Invalid radius keyword: {}", val_kw),
            }
        }
        return Padding {
            top: bottom,
            right: top,
            left: right,
            bottom: left,
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

pub fn parse_value_maybe(value: &String) -> Option<f32> {
    value.parse().ok()
}

pub fn parse_font(value: &String) -> Font {
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
                parse_font_family(&mut family, value);
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

pub fn parse_font_family(family: &mut Family, value: &str) {
    *family = match value {
        "serif" => Family::Serif,
        "fantasy" => Family::Fantasy,
        "cursive" => Family::Cursive,
        "monospace" => Family::Monospace,
        "sans-serif" => Family::SansSerif,
        _ => Family::Serif,
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

pub fn parse_checkbox_icon(value: &str, font: &Font, shaping: Shaping) -> Option<Icon<Font>> {
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
    return Some(Icon {
        font: font.clone(),
        code_point: char,
        size: size,
        line_height: line_height,
        shaping: shaping,
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
