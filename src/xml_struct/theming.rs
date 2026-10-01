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
use log::error;
use std::time::Duration;

use crate::parse_utils::{
    errors::CssValParseError,
    parsers::{
        align::{parse_align_x, parse_align_y},
        background::parse_background,
        center_type::parse_center_type,
        check_anchor::check_anchor,
        checkbox_icon::parse_checkbox_icon,
        color::{parse_color, parse_color_op},
        font::{
            parse_font, parse_font_family, parse_font_stretch, parse_font_style, parse_font_weight,
        },
        line_height::parse_line_height,
        others::{parse_bool, parse_time, parse_value, parse_value_int, parse_value_maybe},
        padding::parse_padding,
        parse_length::parse_length,
        parse_slider_handler::parse_slider_handle_theme,
        radius::parse_radius,
        select_icon::parse_select_icon,
        text::{parse_shaping, parse_text_wrapping},
        text_align::parse_text_alignment,
        two_numbers::parse_two_f32,
        vector::parse_vector,
    },
};

/// A type alias for a vector of font names and their corresponding font paths.
pub type Fonts = Vec<(String, &'static str)>;

/// A struct representing the theme of the XML UI.
/// See the documentation for more details on each field.
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
    pub slider_height: f32, // Height for sliders, since it's not a Length
    pub vertical_slider_width: f32, // Width for vertical sliders, since it's not a Length
    pub slider_rail_width: f32, // Width for slider rails
    pub slider_handle_shape: HandleShape, // Shape for slider handles
    pub table_padding: (f32, f32), // Padding for table cells (x, y)
    pub table_separator_width: (f32, f32), // Width for table separators
    pub center_use_align: bool, // Whether to use align_x and align_y for centering instead of center_all
    pub textarea_min_height: f32, // Minimum height some elements (text editor)
    pub textarea_width: Option<f32>, // Width for text editors, since it's not a Length
    pub toggle_text_alignment: iced::widget::text::Alignment,
    pub toggle_padding_ratio: f32,
    pub tooltip_delay: Duration,
    pub tooltip_gap: f32,
    pub tooltip_padding: f32,
    pub tooltip_no_overflow: bool, // Whether to allow tooltips to overflow the window bounds
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
     * Changes the values of the current theme to match the values of changes only on the properties that are different between first and second.
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
            toggle_text_alignment,
            toggle_padding_ratio,
            tooltip_delay,
            tooltip_gap,
            tooltip_padding,
            tooltip_no_overflow,
            vertical_slider_width,
        });
    }
}

impl Default for XmlTheme {
    /// Returns a default XmlTheme with predefined values for each field.
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
            toggle_text_alignment: iced::widget::text::Alignment::Default,
            toggle_padding_ratio: 0.1,
            tooltip_delay: Duration::from_millis(100),
            tooltip_gap: 0.0,
            tooltip_padding: 5.0,
            tooltip_no_overflow: false,
            vertical_slider_width: 16.0,
        }
    }
}

// handle errors without repetition
macro_rules! herror {
    ($key: expr, $value: expr) => {
        |e| CssValParseError::Value($key.clone(), $value.clone(), e.to_string())
    };
}

pub fn gen_styles_log(key: &String, value: &String, theme: &mut XmlTheme, fonts: &Fonts) {
    match try_gen_styles(key, value, theme, fonts) {
        Ok(_) => {}
        Err(e) => error!("Styling error: {e}"),
    }
}

/// Generates styles for the given key-value pair and updates the provided XmlTheme accordingly.
/// See parse_utils.rs for the parsing functions used to convert string values into the appropriate types.
pub fn try_gen_styles(
    key: &String,
    value: &String,
    theme: &mut XmlTheme,
    fonts: &Fonts,
) -> Result<(), CssValParseError> {
    match key.as_str() {
        "enable" => theme.enable = parse_bool(value).map_err(herror!(key, value))?,
        "snap" => theme.snap = parse_bool(value).map_err(herror!(key, value))?,
        "wrap" => theme.wrap = parse_bool(value).map_err(herror!(key, value))?,
        "clip" => theme.clip = parse_bool(value).map_err(herror!(key, value))?,
        "tooltip-no-overflow" => {
            theme.tooltip_no_overflow = parse_bool(value).map_err(herror!(key, value))?
        }
        "bg" => theme.background = parse_background(value).map_err(herror!(key, value))?,
        "fg-elem" => {
            theme.foreground_element = parse_background(value).map_err(herror!(key, value))?
        }
        "current-item-bg" => {
            theme.selected_background = parse_background(value).map_err(herror!(key, value))?
        }
        "bg-color" => {
            theme.background_color = parse_color_op(value).map_err(herror!(key, value))?
        }
        "fg" => theme.foreground_color = parse_color(value).map_err(herror!(key, value))?,
        "shadow-color" => theme.shadow_color = parse_color(value).map_err(herror!(key, value))?,
        "current-item-fg" => {
            theme.selected_text_color = parse_color(value).map_err(herror!(key, value))?
        }
        "border-color" => theme.border_color = parse_color(value).map_err(herror!(key, value))?,
        "icon-color" => theme.icon_color = parse_color(value).map_err(herror!(key, value))?,
        "placeholder-color" => {
            theme.input_placeholder_color = parse_color(value).map_err(herror!(key, value))?
        }
        "selection-color" => {
            theme.selection_color = parse_color(value).map_err(herror!(key, value))?
        }
        "border-radius" => {
            theme.border_radius = parse_radius(value).map_err(herror!(key, value))?
        }
        "shadow-offset" => {
            theme.shadow_offset = parse_vector(value).map_err(herror!(key, value))?
        }
        "shadow-blur" => {
            theme.shadow_blur_radius = parse_value(value).map_err(herror!(key, value))?
        }
        "border-width" => theme.border_width = parse_value(value).map_err(herror!(key, value))?,
        "grid-width" => theme.grid_width = parse_value(value).map_err(herror!(key, value))?.into(),
        "pane-min-size" => theme.pane_min_size = parse_value(value).map_err(herror!(key, value))?,
        "spacing" => theme.spacing = parse_value(value).map_err(herror!(key, value))?,
        "max_width" => theme.max_width = parse_value(value).map_err(herror!(key, value))?,
        "slider-height" => theme.slider_height = parse_value(value).map_err(herror!(key, value))?,
        "slider-rail-width" => {
            theme.slider_rail_width = parse_value(value).map_err(herror!(key, value))?
        }
        "min-height" => {
            theme.textarea_min_height = parse_value(value).map_err(herror!(key, value))?
        }
        "tooltip-gap" => theme.tooltip_gap = parse_value(value).map_err(herror!(key, value))?,
        "tooltip-padding" => {
            theme.tooltip_padding = parse_value(value).map_err(herror!(key, value))?
        }
        "vertical-slider-width" => {
            theme.vertical_slider_width = parse_value(value).map_err(herror!(key, value))?
        }
        "toggle-padding-ratio" => {
            theme.toggle_padding_ratio = parse_value(value).map_err(herror!(key, value))?
        }
        "max-height" => theme.max_height = parse_value(value).map_err(herror!(key, value))?,
        "scale" => theme.scale = parse_value(value).map_err(herror!(key, value))?,
        "editor-width" => {
            theme.textarea_width = parse_value(value).map_err(herror!(key, value))?.into()
        }
        "grid-responsive-width" => {
            theme.grid_responsive_width = parse_value(value).map_err(herror!(key, value))?.into()
        }
        "font-size" => theme.font_size = parse_value_maybe(value),
        "element-size" => theme.size = parse_value_maybe(value),
        "grid-columns" => {
            theme.grid_columns =
                parse_value_int(value).map_err(herror!(key, value))?.min(1) as usize
        }
        "height" => theme.height = parse_length(value).map_err(herror!(key, value))?,
        "width" => theme.width = parse_length(value).map_err(herror!(key, value))?,
        "select-menu-height" => {
            theme.select_menu_height = parse_length(value).map_err(herror!(key, value))?
        }
        "progress-height" => {
            theme.progress_height =
                Some(parse_length(value).map_err(|e| {
                    CssValParseError::Value(key.clone(), value.clone(), e.to_string())
                })?)
        }
        "padding" => theme.padding = parse_padding(value).map_err(herror!(key, value))?,
        "align-x" => theme.align_x = parse_align_x(value).map_err(herror!(key, value))?,
        "align-y" => theme.align_y = parse_align_y(value).map_err(herror!(key, value))?,
        "font" => theme.font = parse_font(value, fonts).map_err(herror!(key, value))?,
        "font-family" => {
            parse_font_family(&mut theme.font.family, value, fonts).map_err(herror!(key, value))?
        }
        "font-weight" => {
            parse_font_weight(&mut theme.font.weight, value).map_err(herror!(key, value))?
        }
        "font-stretch" => {
            parse_font_stretch(&mut theme.font.stretch, value).map_err(herror!(key, value))?
        }
        "font-style" => {
            parse_font_style(&mut theme.font.style, value).map_err(herror!(key, value))?
        }
        "shaping" => theme.shaping = parse_shaping(value).map_err(herror!(key, value))?,
        "text-wrapping" => {
            theme.text_wrapping = parse_text_wrapping(value).map_err(herror!(key, value))?
        }
        "checkbox-icon" => {
            theme.checkbox_icon = parse_checkbox_icon(value, &theme.font, theme.shaping)
                .map_err(herror!(key, value))?
        }
        "line-height" => {
            theme.line_height = parse_line_height(value).map_err(herror!(key, value))?
        }
        "select-icon" => {
            theme.select_icon =
                parse_select_icon(value, &theme.font).map_err(herror!(key, value))?
        }
        "input-icon" => {
            theme.select_icon =
                parse_select_icon(value, &theme.font).map_err(herror!(key, value))?
        }
        "scroll-anchor" => {
            theme.scroll_anchor = Some(check_anchor(value).map_err(herror!(key, value))?)
        }
        "slider-handle-shape" => {
            theme.slider_handle_shape =
                parse_slider_handle_theme(value, theme).map_err(herror!(key, value))?
        }
        "table-padding" => {
            theme.table_padding = parse_two_f32(value).map_err(herror!(key, value))?
        }
        "table-separator" => {
            theme.table_separator_width = parse_two_f32(value).map_err(herror!(key, value))?
        }
        "center-type" => {
            theme.center_use_align = parse_center_type(value).map_err(herror!(key, value))?
        }
        "toggle-text-alignment" => {
            theme.toggle_text_alignment =
                parse_text_alignment(value).map_err(herror!(key, value))?
        }
        "tooltip-delay" => {
            theme.tooltip_delay =
                Duration::from_secs_f32(parse_time(value).map_err(herror!(key, value))?)
        }
        "center" => {
            theme.center_all = value == "all";
            theme.center_x = value == "x";
            theme.center_y = value == "y";
        }
        _ => {
            return Err(CssValParseError::KeyNotFound(key.clone(), value.clone()));
        }
    };
    return Ok(());
}
