use iced::widget::text;

use crate::{
    dom::{types::DomInternalMessageType, types::QueryResponse},
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener},
    },
    xml_engine::Message,
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};

pub struct Label {
    pub text: String,
}

impl Label {
    pub fn virt(text: String) -> Self {
        Self { text }
    }
}

impl ElementBase for Label {
    fn new(
        xml_element: Box<XmlElement>,
        _: &mut ElementMappings,
        _: i32,
    ) -> Result<Self, ElementError> {
        if xml_element.children.len() > 0 {
            return Err(ElementError::LabelHasChildren(xml_element.clone()));
        }
        return Ok(Self {
            text: xml_element.text.clone(),
        });
    }

    fn render<'a>(
        &self,
        _: &'a ElementsManager,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let text_element = text(self.text.clone());

        let text_element = text_element.style(move |_| iced::widget::text::Style {
            color: Some(theme.foreground_color),
        });

        let mut text_element = text_element
            .height(theme.height)
            .width(theme.width)
            .font(theme.font)
            .shaping(theme.shaping)
            .wrapping(theme.text_wrapping)
            .line_height(theme.line_height)
            .style(move |_| iced::widget::text::Style {
                color: Some(theme.foreground_color),
            });

        if let Some(font_size) = theme.font_size {
            text_element = text_element.size(font_size);
        }

        if theme.center_all {
            text_element = text_element.center();
        }

        return text_element.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(property, new_val) => {
                return match property.as_str() {
                    "text" => {
                        self.text = new_val.clone();
                        Some(ElementEventResponse::success())
                    }
                    _ => None,
                };
            }
            DomInternalMessageType::GetProperty(property) => {
                return match property.as_str() {
                    "text" => Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_str(self.text.clone()),
                    )),
                    _ => None,
                };
            }
            _ => None,
        }
    }
}
