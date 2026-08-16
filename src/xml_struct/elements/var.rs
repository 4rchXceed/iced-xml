// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Var {
    key_name: String,
    fallback_text: String,
}

impl ElementBase for Var {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        if !xml_element.attributes.contains_key("var-key") {
            panic!("<Var /> element must have the var-key attribute")
        }
        let mut fallback_text = String::from("Unknown");
        if xml_element.attributes.contains_key("fallback") {
            fallback_text = xml_element.attributes.get("fallback").unwrap().to_string();
        }
        Self {
            key_name: xml_element.attributes.get("var-key").unwrap().to_string(),
            fallback_text: fallback_text,
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut text = self.fallback_text.clone();
        if datas.child_data.is_some() && datas.child_data.as_ref().unwrap().table_datas.is_some() {
            let text_op = datas
                .child_data
                .unwrap()
                .table_datas
                .unwrap()
                .get(&self.key_name)
                .cloned();
            if text_op.is_some() {
                text = text_op.unwrap();
            }
        } else {
            println!(
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

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut query_response = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(key, value) => match key.as_str() {
                "var-key" => {
                    self.key_name = value.clone();
                    Some((query_response, Vec::new(), Vec::new()))
                }
                "fallback" => {
                    self.fallback_text = value.clone();
                    Some((query_response, Vec::new(), Vec::new()))
                }
                _ => None,
            },
            XmlChangeEvent::GetProperty(key) => match key.as_str() {
                "var-key" => {
                    query_response.data_str = Some(self.key_name.clone());
                    Some((query_response, Vec::new(), Vec::new()))
                }
                "fallback" => {
                    query_response.data_str = Some(self.fallback_text.clone());
                    Some((query_response, Vec::new(), Vec::new()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
