use hex_rgb_converter::HexColor;
use iced::{
    Color, Length, Padding, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
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
