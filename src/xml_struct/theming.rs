use iced::{
    Color, Font, Length, Padding, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{
        button::DEFAULT_PADDING,
        text::{LineHeight, Shaping, Wrapping},
    },
};

use crate::parse_utils::{
    parse_align_x, parse_align_y, parse_checkbox_icon, parse_color, parse_font, parse_font_family,
    parse_font_stretch, parse_font_style, parse_font_weight, parse_length, parse_line_height,
    parse_padding, parse_radius, parse_shaping, parse_text_wrapping, parse_value,
    parse_value_maybe, parse_vector,
};

#[derive(Debug, Clone)]
pub struct XmlTheme {
    pub background_color: Color,
    pub text_color: Color,
    pub snap: bool,
    pub shadow_color: Color,
    pub shadow_blur_radius: f32,
    pub shadow_offset: Vector,
    pub border_color: Color,
    pub border_radius: Radius,
    pub border_width: f32,
    pub clip: bool,
    pub height: Length,
    pub width: Length,
    pub padding: Padding,
    pub spacing: f32,
    pub max_width: f32,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub wrap: bool,
    pub center: bool,
    pub font: Font,
    pub shaping: Shaping,
    pub size: Option<f32>,
    pub font_size: Option<f32>,
    pub text_wrapping: Wrapping,
    pub checkbox_icon: Option<iced::widget::checkbox::Icon<Font>>,
    pub line_height: LineHeight,
}

impl XmlTheme {
    /**
     * Changes the values of the current theme to match the values of `changes` only on the properties that are different between `first` and `second`.
     */
    pub fn apply_only_changes(&mut self, first: &XmlTheme, second: &XmlTheme, changes: &XmlTheme) {
        if first.background_color != second.background_color {
            self.background_color = changes.background_color;
        }
        if first.text_color != second.text_color {
            self.text_color = changes.text_color;
        }
        if first.snap != second.snap {
            self.snap = changes.snap;
        }
        if first.shadow_color != second.shadow_color {
            self.shadow_color = changes.shadow_color;
        }
        if first.shadow_blur_radius != second.shadow_blur_radius {
            self.shadow_blur_radius = changes.shadow_blur_radius;
        }
        if first.shadow_offset != second.shadow_offset {
            self.shadow_offset = changes.shadow_offset;
        }
        if first.border_color != second.border_color {
            self.border_color = changes.border_color;
        }
        if first.border_radius != second.border_radius {
            self.border_radius = changes.border_radius;
        }
        if first.border_width != second.border_width {
            self.border_width = changes.border_width;
        }
        if first.clip != second.clip {
            self.clip = changes.clip;
        }
        if first.height != second.height {
            self.height = changes.height;
        }
        if first.width != second.width {
            self.width = changes.width;
        }
        if first.padding != second.padding {
            self.padding = changes.padding;
        }
        if first.spacing != second.spacing {
            self.spacing = changes.spacing;
        }
        if first.max_width != second.max_width {
            self.max_width = changes.max_width;
        }
        if first.align_x != second.align_x {
            self.align_x = changes.align_x;
        }
        if first.align_y != second.align_y {
            self.align_y = changes.align_y;
        }
        if first.wrap != second.wrap {
            self.wrap = changes.wrap;
        }
        if first.center != second.center {
            self.center = changes.center;
        }
        if first.font != second.font {
            self.font = changes.font;
        }
        if first.shaping != second.shaping {
            self.shaping = changes.shaping;
        }
        if first.size != second.size {
            self.size = changes.size;
        }
        if first.font_size != second.font_size {
            self.font_size = changes.font_size;
        }
        if first.text_wrapping != second.text_wrapping {
            self.text_wrapping = changes.text_wrapping;
        }
        if first.checkbox_icon != second.checkbox_icon {
            self.checkbox_icon = changes.checkbox_icon.clone();
        }
        if first.line_height != second.line_height {
            self.line_height = changes.line_height;
        }
    }
}

impl Default for XmlTheme {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            text_color: Color::BLACK,
            snap: false,
            shadow_color: Color::BLACK,
            shadow_blur_radius: 0.0,
            shadow_offset: Vector { x: 0.0, y: 0.0 },
            border_color: Color::BLACK,
            border_radius: Radius::default(),
            border_width: 0.0,
            clip: false,
            height: Length::Shrink,
            width: Length::Shrink,
            padding: DEFAULT_PADDING,
            max_width: f32::INFINITY,
            spacing: 0.0,
            align_x: Horizontal::Left,
            align_y: Vertical::Top,
            wrap: false,
            center: false,
            font: Font::default(),
            shaping: Shaping::Auto,
            size: None,
            text_wrapping: Wrapping::default(),
            checkbox_icon: None,
            line_height: LineHeight::default(),
            font_size: None,
        }
    }
}

pub fn gen_styles(key: &String, value: &String, theme: &mut XmlTheme) {
    match key.as_str() {
        "bg" => theme.background_color = parse_color(value),
        "fg" => theme.text_color = parse_color(value),
        "snap" => theme.snap = value == "true",
        "shadow-color" => theme.shadow_color = parse_color(value),
        "shadow-blur" => theme.shadow_blur_radius = parse_value(value),
        "shadow-offset" => theme.shadow_offset = parse_vector(value),
        "border-color" => theme.border_color = parse_color(value),
        "border-radius" => theme.border_radius = parse_radius(value),
        "border-width" => theme.border_width = parse_value(value),
        "clip" => theme.clip = value == "true",
        "height" => theme.height = parse_length(value),
        "width" => theme.width = parse_length(value),
        "padding" => theme.padding = parse_padding(value),
        "max_width" => theme.max_width = parse_value(value),
        "align-x" => theme.align_x = parse_align_x(value),
        "spacing" => theme.spacing = parse_value(value),
        "align-y" => theme.align_y = parse_align_y(value),
        "wrap" => theme.wrap = value == "true",
        "center" => theme.center = value == "true",
        "font-family" => parse_font_family(&mut theme.font.family, value),
        "font-weight" => parse_font_weight(&mut theme.font.weight, value),
        "font-stretch" => parse_font_stretch(&mut theme.font.stretch, value),
        "font-style" => parse_font_style(&mut theme.font.style, value),
        "font" => theme.font = parse_font(value),
        "shaping" => theme.shaping = parse_shaping(value),
        "element-size" => theme.size = parse_value_maybe(value),
        "text-wrapping" => theme.text_wrapping = parse_text_wrapping(value),
        "checkbox-icon" => {
            theme.checkbox_icon = parse_checkbox_icon(value, &theme.font, theme.shaping)
        }
        "line-height" => theme.line_height = parse_line_height(value),
        "font-size" => theme.font_size = parse_value_maybe(value),
        _ => {}
    }
}
