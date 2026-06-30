use iced::widget::text;

use crate::{Message, logger::fatal, xml_struct::{elements::{ElementRenderer, element_base::ElementBase}, parser::{XmlElement, XmlTheme}}};

pub struct Label {
    text: String,
    theme: XmlTheme
}

impl ElementBase for Label {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer) -> Self {
        if xml_element.children.len() > 0 {
            fatal("<Label> elements cannot have children");
        }
        Self {
            text: xml_element.text.clone(),
            theme: xml_element.theme.clone(),
        }
    }

    fn render<'a>(&self, _: &'a ElementRenderer) -> iced::Element<'a, Message> {
        let text_element = text(self.text.clone());

        // Theming
        let theme = self.theme.clone();

        let text_element = text_element.style(move |_| iced::widget::text::Style {
            color: Some(theme.text_color),
        });

        return text_element.into();
    }
}
