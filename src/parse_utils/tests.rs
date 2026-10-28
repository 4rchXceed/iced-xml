#[cfg(test)]
mod tests {
    use iced::{
        Background, Color, Font, Length, Padding, Pixels, Vector,
        alignment::{Horizontal, Vertical},
        border::Radius,
        font::{Family, Stretch, Weight},
        gradient::ColorStop,
        widget::{
            slider::HandleShape,
            text::{LineHeight, Shaping, Wrapping},
        },
    };

    use crate::{
        parse_utils::parsers::{
            align::{parse_align_x, parse_align_y},
            background::parse_background,
            center_type::parse_center_type,
            check_anchor::check_anchor,
            checkbox_icon::parse_checkbox_icon,
            color::{parse_color, parse_color_op},
            font::{
                parse_font, parse_font_family, parse_font_stretch, parse_font_style,
                parse_font_weight,
            },
            line_height::parse_line_height,
            others::{parse_bool, parse_time, parse_value, parse_value_int, parse_value_maybe},
            padding::parse_padding,
            pane_axis::parse_pane_axis,
            parse_length::parse_length,
            parse_slider_handler::parse_slider_handle_theme,
            radius::parse_radius,
            text::{parse_shaping, parse_text_wrapping},
            text_align::parse_text_alignment,
            two_numbers::parse_two_f32,
            vector::parse_vector,
        },
        rs_utils::to_rad,
        xml_struct::theming::{Fonts, XmlTheme},
    };

    #[test]
    fn test_parse_length() {
        assert_eq!(
            parse_length(&String::from("1fp")).unwrap(),
            Length::FillPortion(1)
        );
        assert!(parse_length(&String::from("afp")).is_err());
        assert_eq!(
            parse_length(&String::from("1f")).unwrap(),
            Length::Fixed(1.0)
        );
        assert!(parse_length(&String::from("?f")).is_err());
        assert_eq!(parse_length(&String::from("max")).unwrap(), Length::Fill);
        assert_eq!(parse_length(&String::from("min")).unwrap(), Length::Shrink);
        assert!(parse_length(&String::from("Miku")).is_err());
    }

    #[test]
    fn test_parse_color_op() {
        assert!(parse_color_op(&String::from("")).unwrap().is_none());
        assert!(parse_color_op(&String::from("none")).unwrap().is_none());
    }

    #[test]
    fn test_parse_color() {
        assert!(parse_color(&String::from("#")).is_err());
        assert_eq!(parse_color(&String::from("#FFFFFF")).unwrap(), Color::WHITE);
        assert!(parse_color(&String::from("rgba")).is_err());
        assert!(parse_color(&String::from("rgba(0,0,0,1")).is_err());
        assert!(parse_color(&String::from("rgba(0,0,0)")).is_err());
        assert!(parse_color(&String::from("rgba(Miku,Miku,Miku,Miku)")).is_err(),);
        assert_eq!(
            parse_color(&String::from("rgba(128, 90, 20, 0.7)")).unwrap(),
            Color::from_rgba(128.0 / 255.0, 90.0 / 255.0, 20.0 / 255.0, 0.7)
        );
        assert!(parse_color(&String::from("rgb")).is_err());
        assert!(parse_color(&String::from("rgb(0,0,1")).is_err());
        assert!(parse_color(&String::from("rgb(0,0)")).is_err());
        assert!(parse_color(&String::from("rgb(Miku,Miku,Miku)")).is_err());
        assert_eq!(
            parse_color(&String::from("rgb(128, 90, 20)")).unwrap(),
            Color::from_rgb(128.0 / 255.0, 90.0 / 255.0, 20.0 / 255.0)
        );
    }

    #[test]
    fn test_parse_vector() {
        assert!(parse_vector(&String::from("")).is_err());
        assert_eq!(
            parse_vector(&String::from("1.5,3")).unwrap(),
            Vector::new(1.5, 3.0)
        );
        assert!(parse_vector(&String::from("1,Miku")).is_err());
    }

    #[test]
    fn test_parse_radius() {
        assert_eq!(
            parse_radius(&String::from("5.0")).unwrap(),
            Radius::new(5.0)
        );
        assert!(parse_radius(&String::from("Miku")).is_err());
        assert!(parse_radius(&String::from("top_left=")).is_err());
        assert!(parse_radius(&String::from("top_left5.0")).is_err());
        assert!(parse_radius(&String::from("miku=5.0")).is_err());
        assert!(parse_radius(&String::from("top_left=Miku")).is_err());
        assert_eq!(
            parse_radius(&String::from("top_right=5.0")).unwrap(),
            Radius {
                bottom_left: 0.0,
                top_right: 5.0,
                bottom_right: 0.0,
                top_left: 0.0,
            }
        );
        assert_eq!(
            parse_radius(&String::from(
                "top_right=5.0 top_left=3.0 bottom_right=1.0 bottom_left=5.39"
            ))
            .unwrap(),
            Radius {
                bottom_left: 5.39,
                top_right: 5.0,
                bottom_right: 1.0,
                top_left: 3.0,
            }
        );
    }

    #[test]
    fn test_parse_padding() {
        assert_eq!(
            parse_padding(&String::from("5.0")).unwrap(),
            Padding::new(5.0)
        );
        assert!(parse_padding(&String::from("Miku")).is_err());
        assert!(parse_padding(&String::from("top=")).is_err());
        assert!(parse_padding(&String::from("top5.0")).is_err());
        assert!(parse_padding(&String::from("miku=5.0")).is_err());
        assert!(parse_padding(&String::from("top=Miku")).is_err());
        assert_eq!(
            parse_padding(&String::from("right=5.0")).unwrap(),
            Padding {
                bottom: 0.0,
                right: 5.0,
                top: 0.0,
                left: 0.0,
            }
        );
        assert_eq!(
            parse_padding(&String::from("right=5.0 left=3.0 top=1.0 bottom=5.39")).unwrap(),
            Padding {
                bottom: 5.39,
                right: 5.0,
                top: 1.0,
                left: 3.0,
            }
        );
    }

    #[test]
    fn test_parse_align_x() {
        assert_eq!(
            parse_align_x(&String::from("start")).unwrap(),
            Horizontal::Left
        );
        assert_eq!(
            parse_align_x(&String::from("center")).unwrap(),
            Horizontal::Center
        );
        assert_eq!(
            parse_align_x(&String::from("end")).unwrap(),
            Horizontal::Right
        );
        assert!(parse_align_x(&String::from("Miku")).is_err());
    }

    #[test]
    fn test_parse_align_y() {
        assert_eq!(
            parse_align_y(&String::from("start")).unwrap(),
            Vertical::Top
        );
        assert_eq!(
            parse_align_y(&String::from("center")).unwrap(),
            Vertical::Center
        );
        assert_eq!(
            parse_align_y(&String::from("end")).unwrap(),
            Vertical::Bottom
        );
        assert!(parse_align_y(&String::from("Miku")).is_err());
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value(&String::from("3.9")).unwrap(), 3.9);
        assert!(parse_value(&String::from("Miku")).is_err());
    }

    #[test]
    fn test_parse_value_int() {
        assert_eq!(parse_value_int(&String::from("5")).unwrap(), 5);
        assert!(parse_value_int(&String::from("Miku")).is_err());
    }

    #[test]
    fn test_parse_value_maybe() {
        assert_eq!(parse_value_maybe(&String::from("3.9")), Some(3.9));
        assert_eq!(parse_value_maybe(&String::from("Miku")), None);
    }

    #[test]
    fn test_parse_font_style() {
        let mut style = iced::font::Style::Normal;

        assert!(parse_font_style(&mut style, "normal").is_ok());
        assert_eq!(style, iced::font::Style::Normal);
        assert!(parse_font_style(&mut style, "italic").is_ok());
        assert_eq!(style, iced::font::Style::Italic);
        assert!(parse_font_style(&mut style, "oblique").is_ok());
        assert_eq!(style, iced::font::Style::Oblique);
        assert!(parse_font_style(&mut style, "Miku").is_err());
    }

    #[test]
    fn test_parse_font_stretch() {
        let mut stretch = Stretch::Normal;

        assert!(parse_font_stretch(&mut stretch, "normal").is_ok());
        assert_eq!(stretch, Stretch::Normal);
        assert!(parse_font_stretch(&mut stretch, "condensed").is_ok());
        assert_eq!(stretch, Stretch::Condensed);
        assert!(parse_font_stretch(&mut stretch, "expanded").is_ok());
        assert_eq!(stretch, Stretch::Expanded);
        assert!(parse_font_stretch(&mut stretch, "extra-condensed").is_ok());
        assert_eq!(stretch, Stretch::ExtraCondensed);
        assert!(parse_font_stretch(&mut stretch, "extra-expanded").is_ok());
        assert_eq!(stretch, Stretch::ExtraExpanded);
        assert!(parse_font_stretch(&mut stretch, "semi-condensed").is_ok());
        assert_eq!(stretch, Stretch::SemiCondensed);
        assert!(parse_font_stretch(&mut stretch, "semi-expanded").is_ok());
        assert_eq!(stretch, Stretch::SemiExpanded);
        assert!(parse_font_stretch(&mut stretch, "ultra-condensed").is_ok());
        assert_eq!(stretch, Stretch::UltraCondensed);
        assert!(parse_font_stretch(&mut stretch, "ultra-expanded").is_ok());
        assert_eq!(stretch, Stretch::UltraExpanded);
        assert!(parse_font_stretch(&mut stretch, "Miku").is_err());
    }

    #[test]
    fn test_parse_font_weight() {
        let mut weight = Weight::Normal;

        assert!(parse_font_weight(&mut weight, "normal").is_ok());
        assert_eq!(weight, Weight::Normal);
        assert!(parse_font_weight(&mut weight, "bold").is_ok());
        assert_eq!(weight, Weight::Bold);
        assert!(parse_font_weight(&mut weight, "black").is_ok());
        assert_eq!(weight, Weight::Black);
        assert!(parse_font_weight(&mut weight, "extra-bold").is_ok());
        assert_eq!(weight, Weight::ExtraBold);
        assert!(parse_font_weight(&mut weight, "extra-light").is_ok());
        assert_eq!(weight, Weight::ExtraLight);
        assert!(parse_font_weight(&mut weight, "light").is_ok());
        assert_eq!(weight, Weight::Light);
        assert!(parse_font_weight(&mut weight, "medium").is_ok());
        assert_eq!(weight, Weight::Medium);
        assert!(parse_font_weight(&mut weight, "semibold").is_ok());
        assert_eq!(weight, Weight::Semibold);
        assert!(parse_font_weight(&mut weight, "thin").is_ok());
        assert_eq!(weight, Weight::Thin);
        assert!(parse_font_weight(&mut weight, "Miku").is_err());
    }

    #[test]
    fn test_parse_font_family() {
        let mut family = Family::Serif;
        let mut fonts = Fonts::new();

        fonts.push(("Miku".to_string(), "Miku"));

        assert!(parse_font_family(&mut family, "serif", &fonts).is_ok());
        assert_eq!(family, Family::Serif);
        assert!(parse_font_family(&mut family, "fantasy", &fonts).is_ok());
        assert_eq!(family, Family::Fantasy);
        assert!(parse_font_family(&mut family, "cursive", &fonts).is_ok());
        assert_eq!(family, Family::Cursive);
        assert!(parse_font_family(&mut family, "monospace", &fonts).is_ok());
        assert_eq!(family, Family::Monospace);
        assert!(parse_font_family(&mut family, "sans-serif", &fonts).is_ok());
        assert_eq!(family, Family::SansSerif);
        assert!(parse_font_family(&mut family, "Miku", &fonts).is_ok());
        assert_eq!(family, Family::Name("Miku"));
        assert!(parse_font_family(&mut family, "Unknown", &fonts).is_err());
    }

    #[test]
    fn test_parse_font() {
        let mut fonts = Fonts::new();

        fonts.push(("Miku".to_string(), "Miku"));

        let font = parse_font(
            &String::from("family=Miku weight=bold stretch=condensed style=italic"),
            &fonts,
        )
        .unwrap();

        assert_eq!(font.family, Family::Name("Miku"));
        assert_eq!(font.weight, Weight::Bold);
        assert_eq!(font.stretch, Stretch::Condensed);
        assert_eq!(font.style, iced::font::Style::Italic);
    }

    #[test]
    fn test_parse_shaping() {
        assert_eq!(parse_shaping("quality").unwrap(), Shaping::Advanced);
        assert_eq!(parse_shaping("performance").unwrap(), Shaping::Basic);
        assert_eq!(parse_shaping("auto").unwrap(), Shaping::Auto);
        assert!(parse_shaping("Miku").is_err());
    }

    #[test]
    fn test_parse_text_wrapping() {
        assert_eq!(parse_text_wrapping("word").unwrap(), Wrapping::Word);
        assert_eq!(parse_text_wrapping("glyph").unwrap(), Wrapping::Glyph);
        assert_eq!(
            parse_text_wrapping("word-or-glyph").unwrap(),
            Wrapping::WordOrGlyph
        );
        assert_eq!(parse_text_wrapping("none").unwrap(), Wrapping::None);
        assert!(parse_text_wrapping("Miku").is_err());
    }

    #[test]
    fn test_parse_checkbox_icon() {
        let font = Font {
            family: Family::Serif,
            weight: Weight::Normal,
            stretch: Stretch::Normal,
            style: iced::font::Style::Normal,
        };

        let shaping = Shaping::Auto;

        assert!(
            parse_checkbox_icon("none", &font, shaping)
                .unwrap()
                .is_none()
        );
        assert!(parse_checkbox_icon("", &font, shaping).is_err());
        assert!(parse_checkbox_icon("\"X\" 20", &font, shaping).is_err());
        assert!(parse_checkbox_icon("X\" 20 absolute 15", &font, shaping).is_err());
        assert!(parse_checkbox_icon("\"X 20 absolute 15", &font, shaping).is_err());
        assert!(parse_checkbox_icon("X 20 absolute 15", &font, shaping).is_err());
        assert!(parse_checkbox_icon("\"X\" Miku", &font, shaping).is_err());
        assert_eq!(
            parse_checkbox_icon("\"X\" absolute 15", &font, shaping)
                .unwrap()
                .unwrap(),
            iced::widget::checkbox::Icon {
                font: font.clone(),
                code_point: 'X',
                size: None,
                line_height: LineHeight::Absolute(Pixels(15.0)),
                shaping: shaping,
            }
        );
        assert_eq!(
            parse_checkbox_icon("\"❌\" 20 relative 15.5", &font, shaping)
                .unwrap()
                .unwrap(),
            iced::widget::checkbox::Icon {
                font: font.clone(),
                code_point: '❌',
                size: Some(Pixels(20.0)),
                line_height: LineHeight::Relative(15.5),
                shaping: shaping,
            }
        );
    }

    #[test]
    fn test_parse_line_height() {
        assert_eq!(
            parse_line_height("absolute 15").unwrap(),
            LineHeight::Absolute(Pixels(15.0))
        );
        assert_eq!(
            parse_line_height("relative 1.5").unwrap(),
            LineHeight::Relative(1.5)
        );
        assert!(parse_line_height("absolute Miku").is_err());
    }

    #[test]
    fn test_parse_pane_axis() {
        assert_eq!(
            parse_pane_axis("horizontal").unwrap(),
            iced::widget::pane_grid::Axis::Horizontal
        );
        assert!(parse_pane_axis("Miku").is_err());
        assert_eq!(
            parse_pane_axis("vertical").unwrap(),
            iced::widget::pane_grid::Axis::Vertical
        );
    }

    #[test]
    fn test_check_anchor() {
        assert_eq!(check_anchor("top").unwrap(), "top");
        assert_eq!(check_anchor("bottom").unwrap(), "bottom");
        assert_eq!(check_anchor("left").unwrap(), "left");
        assert_eq!(check_anchor("right").unwrap(), "right");
        assert!(check_anchor("Miku").is_err());
    }

    #[test]
    fn test_parse_slider_handle_theme() {
        let theme = XmlTheme {
            border_radius: Radius {
                top_left: 1.0,
                top_right: 1.0,
                bottom_right: 1.0,
                bottom_left: 1.0,
            },
            ..Default::default()
        };
        assert!(parse_slider_handle_theme("", &theme).is_err());
        assert!(parse_slider_handle_theme("Miku 20", &theme).is_err());
        assert!(parse_slider_handle_theme("circle Miku", &theme).is_err());
        assert_eq!(
            parse_slider_handle_theme("circle 20", &theme).unwrap(),
            HandleShape::Circle { radius: 20.0 }
        );
        assert!(parse_slider_handle_theme("rectangle 1.5", &theme).is_err());
        assert_eq!(
            parse_slider_handle_theme("rectangle 2", &theme).unwrap(),
            HandleShape::Rectangle {
                width: 2,
                border_radius: theme.border_radius,
            }
        );
    }

    #[test]
    fn test_parse_two_f32() {
        assert_eq!(parse_two_f32("3.9 4.2").unwrap(), (3.9, 4.2));
        assert!(parse_two_f32("Miku 4.2").is_err());
        assert!(parse_two_f32("3.9 Miku").is_err());
        assert_eq!(parse_two_f32("3.9").unwrap(), (3.9, 3.9));
        assert!(parse_two_f32("Miku").is_err());
    }

    #[test]
    fn test_parse_center_type() {
        assert_eq!(parse_center_type("align").unwrap(), true);
        assert_eq!(parse_center_type("center").unwrap(), false);
        assert!(parse_center_type("Miku").is_err());
    }

    #[test]
    fn test_parse_background() {
        assert_eq!(
            parse_background("#FFFF00").unwrap(),
            Background::Color(Color::from_rgb(1.0, 1.0, 0.0))
        );
        assert_eq!(
            parse_background("linear-gradient(45deg, #FF0000 0%, #00FF00 50%, #0000FF 100%)")
                .unwrap(),
            Background::Gradient(iced::Gradient::Linear(iced::gradient::Linear {
                angle: iced::Radians(to_rad(45.0)),
                stops: [
                    Some(ColorStop {
                        color: Color::from_rgb(1.0, 0.0, 0.0),
                        offset: 0.0,
                    }),
                    Some(ColorStop {
                        color: Color::from_rgb(0.0, 1.0, 0.0),
                        offset: 0.5,
                    }),
                    Some(ColorStop {
                        color: Color::from_rgb(0.0, 0.0, 1.0),
                        offset: 1.0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            }))
        );
        assert!(
            parse_background(
                "linear-gradient(90,rgba(42, 123, 155, 1) 0%, rgba(87, 199, 133, 1) 50%, rgba(237, 221, 83, 1) 100%)"
            ).is_err()
        );
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true").unwrap(), true);
        assert_eq!(parse_bool("false").unwrap(), false);
        assert!(parse_bool("Miku").is_err());
    }

    #[test]
    fn test_parse_text_alignment() {
        assert_eq!(
            parse_text_alignment("left").unwrap(),
            iced::widget::text::Alignment::Left
        );
        assert_eq!(
            parse_text_alignment("center").unwrap(),
            iced::widget::text::Alignment::Center
        );
        assert_eq!(
            parse_text_alignment("right").unwrap(),
            iced::widget::text::Alignment::Right
        );
        assert_eq!(
            parse_text_alignment("justified").unwrap(),
            iced::widget::text::Alignment::Justified
        );
        assert_eq!(
            parse_text_alignment("default").unwrap(),
            iced::widget::text::Alignment::Default
        );
        assert!(parse_text_alignment("Miku").is_err());
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("1000ms").unwrap(), 1.0);
        assert_eq!(parse_time("1s").unwrap(), 1.0);
        assert!(parse_time("Miku").is_err());
    }
}
