
use iced::{Background, Border, Shadow, Vector, border::Radius};

use crate::{Message, logger::fatal, xml_struct::{elements::{ElementRenderer, element_base::ElementBase}, parser::{XmlElement, XmlTheme}}};

pub struct Button {
    children: Vec<i32>,
    theme: XmlTheme
}

impl ElementBase for Button {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        if !xml_element.text.is_empty() {
            if xml_element.children.is_empty() {
                let mut children: Vec<i32> = Vec::new();
                children.push(renderer.init_element(&XmlElement { tag: "Label".to_string(), attributes: Vec::new(), text: xml_element.text.clone(), children: Vec::new(), theme: xml_element.theme.clone() })); // Auto-create Label element
               Self {
                   children: children,
                   theme: xml_element.theme.clone()
                }
            } else {
                fatal(format!("Button with text cannot have children: {}", xml_element.text).as_str());
                Self {
                    children: Vec::new(),
                    theme: xml_element.theme.clone()
                }
            }
        } else {
            let mut children: Vec<i32> = Vec::new();
            for child in &xml_element.children {
                children.push(renderer.init_element(child));
            }
            Self {
                children,
                theme: xml_element.theme.clone()
            }
        }
    }
    fn render<'a>(&self, renderer: &'a ElementRenderer) -> iced::Element<'a, Message> {
        let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();
        for child in &self.children {
            container = container.push(renderer.render_element(*child));
        }

        let button: iced::widget::Button<'a, Message> = iced::widget::Button::new(container);

        let theme = self.theme.clone();

        let button = button.style(move |_, _| iced::widget::button::Style {
            background: Some(Background::Color(theme.background_color)),
            text_color: theme.text_color,
            border: Border {
                color: theme.border_color,
                radius: theme.border_radius,
                width: theme.border_width,
                ..Border::default()
            },
            shadow:  Shadow {
                color: theme.shadow_color,
                blur_radius: theme.shadow_blur_radius,
                offset: theme.shadow_offset,
                ..Shadow::default()
            },
            snap: theme.snap,
            ..Default::default()
        });

        return button.into();
    }
}
