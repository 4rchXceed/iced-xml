use iced::{
    Color, Font, Length, Padding, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{
        button::DEFAULT_PADDING,
        text::{LineHeight, Shaping, Wrapping},
        text_input::Side,
    },
};

use crate::parse_utils::{
    check_anchor, parse_align_x, parse_align_y, parse_checkbox_icon, parse_color, parse_font,
    parse_font_family, parse_font_stretch, parse_font_style, parse_font_weight, parse_length,
    parse_line_height, parse_padding, parse_radius, parse_select_icon, parse_shaping,
    parse_text_wrapping, parse_value, parse_value_int, parse_value_maybe, parse_vector,
};

// The theme struct
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
    pub center_all: bool,
    pub font: Font,
    pub shaping: Shaping,
    pub size: Option<f32>,
    pub font_size: Option<f32>,
    pub text_wrapping: Wrapping,
    pub checkbox_icon: Option<iced::widget::checkbox::Icon<Font>>,
    pub line_height: LineHeight,
    pub select_icon: Option<iced::widget::text_input::Icon<Font>>,
    pub icon_color: Color,
    pub input_placeholder_color: Color,
    pub selection_color: Color,
    pub select_menu_height: Length,
    pub selected_background_color: Color,
    pub selected_text_color: Color,
    pub center_x: bool,
    pub center_y: bool,
    pub max_height: f32,
    pub scale: f32,
    pub grid_columns: usize,
    pub grid_responsive_width: Option<f32>,
    pub grid_width: Option<f32>,
    pub pane_min_size: f32, // Minimum size for panes in a pane grid
    pub scroll_anchor: Option<String>,
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
        if first.center_all != second.center_all {
            self.center_all = changes.center_all;
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
        if first.select_icon.is_some() && second.select_icon.is_some() {
            let a = first.select_icon.clone().unwrap();
            let b = second.select_icon.clone().unwrap();
            let side_a = match a.side {
                Side::Left => "l",
                Side::Right => "r",
            };
            let side_b = match a.side {
                Side::Left => "l",
                Side::Right => "r",
            };
            if a.code_point == b.code_point
                && a.font == b.font
                && a.size == b.size
                && a.spacing == b.spacing
                && side_a == side_b
            {}
        }
        if first.icon_color != second.icon_color {
            self.icon_color = changes.icon_color;
        }
        if first.input_placeholder_color != second.input_placeholder_color {
            self.input_placeholder_color = changes.input_placeholder_color;
        }
        if first.selection_color != second.selection_color {
            self.selection_color = changes.selection_color;
        }
        if first.select_menu_height != second.select_menu_height {
            self.select_menu_height = changes.select_menu_height;
        }
        if first.selected_background_color != second.selected_background_color {
            self.selected_background_color = changes.selected_background_color;
        }
        if first.selected_text_color != second.selected_text_color {
            self.selected_text_color = changes.selected_text_color;
        }
        if first.center_x != second.center_x {
            self.center_x = changes.center_x;
        }
        if first.center_y != second.center_y {
            self.center_y = changes.center_y;
        }
        if first.max_height != second.max_height {
            self.max_height = changes.max_height;
        }
        if first.scale != second.scale {
            self.scale = changes.scale;
        }
        if first.grid_columns != second.grid_columns {
            self.grid_columns = changes.grid_columns;
        }
        if first.grid_responsive_width != second.grid_responsive_width {
            self.grid_responsive_width = changes.grid_responsive_width;
        }
        if first.grid_width != second.grid_width {
            self.grid_width = changes.grid_width;
        }
        if first.pane_min_size != second.pane_min_size {
            self.pane_min_size = changes.pane_min_size;
        }
        if first.scroll_anchor != second.scroll_anchor {
            self.scroll_anchor = changes.scroll_anchor.clone();
        }
    }
}

impl Default for XmlTheme {
    fn default() -> Self {
        Self {
            background_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
            snap: false,
            shadow_color: Color::TRANSPARENT,
            shadow_blur_radius: 0.0,
            shadow_offset: Vector { x: 0.0, y: 0.0 },
            border_color: Color::BLACK,
            border_radius: Radius::default(),
            border_width: 0.0,
            clip: false,
            height: Length::Shrink,
            width: Length::FillPortion(1),
            padding: DEFAULT_PADDING,
            max_width: f32::INFINITY,
            spacing: 0.0,
            align_x: Horizontal::Left,
            align_y: Vertical::Top,
            wrap: false,
            center_all: false,
            font: Font::default(),
            shaping: Shaping::Auto,
            size: None,
            text_wrapping: Wrapping::default(),
            checkbox_icon: None,
            line_height: LineHeight::default(),
            font_size: None,
            select_icon: None,
            icon_color: Color::BLACK,
            input_placeholder_color: Color::BLACK,
            selection_color: Color::BLACK,
            select_menu_height: Length::Shrink,
            selected_background_color: Color::from_rgb(0.8, 0.8, 0.8),
            selected_text_color: Color::BLACK,
            center_x: false,
            center_y: false,
            max_height: f32::INFINITY,
            scale: 1.0,
            grid_columns: 1,
            grid_responsive_width: None,
            grid_width: None,
            pane_min_size: 50.0,
            scroll_anchor: None,
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
        "center" => {
            theme.center_all = value == "all";
            theme.center_x = value == "x";
            theme.center_y = value == "y";
        }
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
        "select-icon" => theme.select_icon = parse_select_icon(value, &theme.font),
        "icon-color" => theme.icon_color = parse_color(value),
        "input-placeholder-color" => theme.input_placeholder_color = parse_color(value),
        "input-selection-color" => theme.selection_color = parse_color(value),
        "select-menu-height" => theme.select_menu_height = parse_length(value),
        "current-item-bg" => theme.selected_background_color = parse_color(value),
        "current-item-fg" => theme.selected_text_color = parse_color(value),
        "max-height" => theme.max_height = parse_value(value),
        "scale" => theme.scale = parse_value(value),
        "grid-columns" => theme.grid_columns = parse_value_int(value).min(1) as usize,
        "grid-responsive-width" => theme.grid_responsive_width = parse_value(value).into(),
        "grid-width" => theme.grid_width = parse_value(value).into(),
        "pane-min-size" => theme.pane_min_size = parse_value(value),
        "scroll-anchor" => theme.scroll_anchor = Some(check_anchor(value)),
        _ => {
            println!("Unknown theme property: {} = {}", key, value);
        }
    }
}
