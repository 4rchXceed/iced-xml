use iced::{
    Background, Color, Font, Length, Padding, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{
        button::DEFAULT_PADDING,
        slider::HandleShape,
        text::{LineHeight, Shaping, Wrapping},
        text_input::Side,
    },
};

use crate::parse_utils::{
    check_anchor, parse_align_x, parse_align_y, parse_background, parse_bool, parse_center_type,
    parse_checkbox_icon, parse_color, parse_color_op, parse_font, parse_font_family,
    parse_font_stretch, parse_font_style, parse_font_weight, parse_length, parse_line_height,
    parse_padding, parse_radius, parse_select_icon, parse_shaping, parse_slider_handle_theme,
    parse_text_wrapping, parse_two_f32, parse_value, parse_value_int, parse_value_maybe,
    parse_vector,
};

// The theme struct
#[derive(Debug, Clone)]
pub struct XmlTheme {
    pub enable: bool,
    pub background: Background,
    pub background_color: Option<Color>,
    pub foreground_color: Color,
    pub foreground_element: Background,
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
    pub selected_background: Background,
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
    pub progress_height: Option<Length>,
    pub slider_height: f32,     // Height for sliders, since it's not a Length
    pub slider_rail_width: f32, // Width for slider rails
    pub slider_handle_shape: HandleShape, // Shape for slider handles
    pub table_padding: (f32, f32), // Padding for table cells (x, y)
    pub table_separator_width: (f32, f32), // Width for table separators
    pub center_use_align: bool, // Whether to use align_x and align_y for centering instead of center_all
    pub textarea_min_height: f32, // Minimum height some elements (text editor)
    pub textarea_width: Option<f32>, // Width for text editors, since it's not a Length
}

macro_rules! check {
    ($first: expr, $second: expr, $changes: expr, $self: expr,  { $($name:ident),* $(,)? }) => {
        $(
            if $first.$name != $second.$name {
                $self.$name = $changes.$name;
            }
        )*
    };
}

impl XmlTheme {
    /**
     * Changes the values of the current theme to match the values of `changes` only on the properties that are different between `first` and `second`.
     */
    pub fn apply_only_changes(&mut self, first: &XmlTheme, second: &XmlTheme, changes: &XmlTheme) {
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
        if first.scroll_anchor != second.scroll_anchor {
            self.scroll_anchor = changes.scroll_anchor.clone();
        }
        if first.slider_handle_shape != second.slider_handle_shape {
            self.slider_handle_shape = changes.slider_handle_shape.clone();
        }
        if first.checkbox_icon != second.checkbox_icon {
            self.checkbox_icon = changes.checkbox_icon.clone();
        }
        check!(first, second, changes, self, {
            enable,
            background,
            foreground_color,
            snap,
            shadow_color,
            shadow_blur_radius,
            shadow_offset,
            border_color,
            border_radius,
            border_width,
            clip,
            height,
            width,
            padding,
            spacing,
            max_width,
            align_x,
            align_y,
            wrap,
            center_all,
            font,
            shaping,
            size,
            font_size,
            text_wrapping,
            line_height,
            icon_color,
            input_placeholder_color,
            selection_color,
            select_menu_height,
            selected_background,
            selected_text_color,
            center_x,
            center_y,
            max_height,
            scale,
            grid_columns,
            grid_responsive_width,
            grid_width,
            pane_min_size,
            progress_height,
            slider_height,
            slider_rail_width,
            table_padding,
            table_separator_width,
            table_separator_width,
            center_use_align,
            foreground_element,
            enable,
            background_color,
            textarea_min_height,
            textarea_width,
        });
    }
}

impl Default for XmlTheme {
    fn default() -> Self {
        Self {
            enable: true,
            background_color: None,
            background: Background::Color(Color::TRANSPARENT),
            foreground_color: Color::BLACK,
            foreground_element: Background::Color(Color::TRANSPARENT),
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
            selected_background: Background::Color(Color::from_rgb(0.8, 0.8, 0.8)),
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
            progress_height: None,
            slider_height: 16.0,
            slider_rail_width: 4.0,
            slider_handle_shape: HandleShape::Circle { radius: 8.0 },
            table_padding: (0.0, 0.0),
            table_separator_width: (1.0, 1.0),
            center_use_align: false,
            textarea_min_height: 0.0,
            textarea_width: None,
        }
    }
}

pub fn gen_styles(key: &String, value: &String, theme: &mut XmlTheme) {
    match key.as_str() {
        "enable" => theme.enable = parse_bool(value),
        "bg" => theme.background = parse_background(value),
        "fg-elem" => theme.foreground_element = parse_background(value),
        "bg-color" => theme.background_color = parse_color_op(value),
        "fg" => theme.foreground_color = parse_color(value),
        "snap" => theme.snap = parse_bool(value),
        "shadow-color" => theme.shadow_color = parse_color(value),
        "shadow-blur" => theme.shadow_blur_radius = parse_value(value),
        "shadow-offset" => theme.shadow_offset = parse_vector(value),
        "border-color" => theme.border_color = parse_color(value),
        "border-radius" => theme.border_radius = parse_radius(value),
        "border-width" => theme.border_width = parse_value(value),
        "clip" => theme.clip = parse_bool(value),
        "height" => theme.height = parse_length(value),
        "width" => theme.width = parse_length(value),
        "padding" => theme.padding = parse_padding(value),
        "max_width" => theme.max_width = parse_value(value),
        "align-x" => theme.align_x = parse_align_x(value),
        "spacing" => theme.spacing = parse_value(value),
        "align-y" => theme.align_y = parse_align_y(value),
        "wrap" => theme.wrap = parse_bool(value),
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
        "input-icon" => theme.select_icon = parse_select_icon(value, &theme.font),
        "icon-color" => theme.icon_color = parse_color(value),
        "placeholder-color" => theme.input_placeholder_color = parse_color(value),
        "selection-color" => theme.selection_color = parse_color(value),
        "select-menu-height" => theme.select_menu_height = parse_length(value),
        "current-item-bg" => theme.selected_background = parse_background(value),
        "current-item-fg" => theme.selected_text_color = parse_color(value),
        "max-height" => theme.max_height = parse_value(value),
        "scale" => theme.scale = parse_value(value),
        "grid-columns" => theme.grid_columns = parse_value_int(value).min(1) as usize,
        "grid-responsive-width" => theme.grid_responsive_width = parse_value(value).into(),
        "grid-width" => theme.grid_width = parse_value(value).into(),
        "pane-min-size" => theme.pane_min_size = parse_value(value),
        "scroll-anchor" => theme.scroll_anchor = Some(check_anchor(value)),
        "progress-height" => theme.progress_height = Some(parse_length(value)),
        "slider-height" => theme.slider_height = parse_value(value),
        "slider-rail-width" => theme.slider_rail_width = parse_value(value),
        "slider-handle-shape" => {
            theme.slider_handle_shape = parse_slider_handle_theme(value, theme)
        }
        "table-padding" => theme.table_padding = parse_two_f32(value),
        "table-separator" => theme.table_separator_width = parse_two_f32(value),
        "center-type" => theme.center_use_align = parse_center_type(value),
        "min-height" => theme.textarea_min_height = parse_value(value),
        "editor-width" => theme.textarea_width = parse_value(value).into(),
        _ => {
            println!("Unknown theme property: {} = {}", key, value);
        }
    }
}
