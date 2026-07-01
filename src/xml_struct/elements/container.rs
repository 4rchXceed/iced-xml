use crate::{
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub struct Container {
    children: Vec<i32>,
}

impl ElementBase for Container {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        let mut children: Vec<i32> = Vec::new();
        for child in &xml_element.children {
            children.push(renderer.init_element(child));
        }
        Self { children: children }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        _: &'a XmlTheme,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();

        for child in &self.children {
            container = container.push(renderer.render_element(*child));
        }

        // Column doesn't have theming. TODO: Add a parent container that actually supports theming.

        return container.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) {
        match event {
            _ => (),
        }
    }
}
