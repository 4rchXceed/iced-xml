use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Row {
    children: Vec<i32>,
}

impl ElementBase for Row {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, _: i32) -> Self {
        let mut children: Vec<i32> = Vec::new();
        for child in &xml_element.children {
            children.push(renderer.init_element_from_xml(child));
        }
        Self { children: children }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: &'a ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut container: iced::widget::Row<'a, Message> = iced::widget::Row::new();

        for child in &self.children {
            container = container.push(renderer.render_element(*child));
        }

        container = container
            .clip(theme.clip)
            .height(theme.height)
            .padding(theme.padding)
            .width(theme.width)
            .spacing(theme.spacing)
            .align_y(theme.align_y);
        if theme.wrap {
            return container.wrap().into();
        }

        return container.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            _ => None,
        }
    }
}
