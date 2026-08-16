use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Col {
    children: Vec<i32>,
}

impl ElementBase for Col {
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
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();

        for child in &self.children {
            container = container.push(renderer.render_element(*child, datas.child_data.clone()));
        }

        container = container
            .clip(theme.clip)
            .height(theme.height)
            .padding(theme.padding)
            .width(theme.width)
            .max_width(theme.max_width)
            .spacing(theme.spacing)
            .align_x(theme.align_x);
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
