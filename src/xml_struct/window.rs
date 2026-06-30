

use crate::{Message, logger::fatal, xml_struct::{elements::{ElementRenderer}, parser::XmlElement}};

pub struct XmlWindow {
    title: String,
    element_renderer: ElementRenderer,
    root: XmlElement,
    root_uid: i32
}

impl XmlWindow {
    pub fn new(root: XmlElement) -> Self {
        if root.tag != "Window" {
            fatal("Root element must be a <Window></Window>");
        }
        let mut element_renderer = ElementRenderer::new();
        let uid = element_renderer.init_element(&root);

        Self { title: String::new(), element_renderer: element_renderer, root: root, root_uid: uid }
    }

    pub fn render(&self) -> iced::Element<'_, Message> {
        return self.element_renderer.render_element(self.root_uid).into();
    }
}
