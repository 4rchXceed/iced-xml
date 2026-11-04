use log::warn;

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

pub struct Var {
    key_name: String,
    fallback_text: String,
}

impl ElementBase for Var {
    fn new(
        xml_element: Box<XmlElement>,
        _: &mut ElementMappings,
        _: i32,
    ) -> Result<Self, ElementError> {
        if !xml_element.attributes.contains_key("var-key") {
            return Err(ElementError::VarMustHaveVarKey(xml_element.clone()));
        }

        let mut fallback_text = String::from("Unknown");

        if let Some(fallback_attr) = xml_element.attributes.get("fallback") {
            fallback_text = fallback_attr.to_string();
        }

        return Ok(Self {
            key_name: xml_element.attributes.get("var-key").unwrap().to_string(),
            fallback_text: fallback_text,
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
        let mut text = self.fallback_text.clone();

        if let Some(child_data) = datas.child_data
            && let Some(table_datas) = child_data.table_datas
        {
            let text_op = table_datas.get(&self.key_name).cloned();

            if let Some(text_op) = text_op {
                text = text_op;
            }
        } else {
            warn!(
                "<Var /> element is not a child of a var-providing element like <Table />. This is almost certainly a mistake"
            )
        }

        let text_element = iced::widget::text(text);

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
            DomInternalMessageType::PropertyChange(key, value) => match key.as_str() {
                "var-key" => {
                    self.key_name = value.clone();

                    Some(ElementEventResponse::success())
                }
                "fallback" => {
                    self.fallback_text = value.clone();

                    Some(ElementEventResponse::success())
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(key) => match key.as_str() {
                "var-key" => Some(ElementEventResponse::new(
                    QueryResponse::success().with_data_str(self.key_name.clone()),
                )),
                "fallback" => Some(ElementEventResponse::new(
                    QueryResponse::success().with_data_str(self.fallback_text.clone()),
                )),
                _ => None,
            },
            _ => None,
        }
    }
}
