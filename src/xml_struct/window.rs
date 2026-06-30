use crate::{
    logger::fatal,
    xml_engine::Message,
    xml_struct::{elements::ElementRenderer, parser::XmlElement},
};

pub struct XmlWindow {
    title: String,
    root: XmlElement,
    root_uid: i32,
    pub element_renderer: ElementRenderer,
}

impl XmlWindow {
    pub fn new(root: XmlElement) -> Self {
        if root.tag != "Window" {
            fatal("Root element must be a <Window></Window>");
        }
        let mut element_renderer = ElementRenderer::new();
        let uid = element_renderer.init_element(&root);

        Self {
            title: String::new(),
            root: root,
            root_uid: uid,
            element_renderer: element_renderer,
        }
    }

    pub fn render(&self) -> iced::Element<'_, Message> {
        return self.element_renderer.render_element(self.root_uid).into();
    }

    pub fn emit_event(&mut self, event_uid: i32) {
        for event_listener in self.element_renderer.event_listeners.iter_mut() {
            if event_listener.event_uid == event_uid {}
        }
    }
}
