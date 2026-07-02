use crate::{
    dom::query::EventResponse,
    xml_engine::Message,
    xml_struct::{elements::ElementRenderer, parser::XmlElement},
};

pub struct XmlWindow {
    // title: String,
    // root: XmlElement,
    root_uid: i32,
    pub element_renderer: ElementRenderer,
    pub fired_events: Vec<(i32, EventResponse)>,
}

impl XmlWindow {
    pub fn new(root: XmlElement) -> Self {
        if root.tag != "Window" {
            panic!("Root element must be a <Window></Window>");
        }
        let mut element_renderer = ElementRenderer::new();
        let uid = element_renderer.init_element_from_xml(&root);

        Self {
            // title: String::new(),
            // root: root,
            root_uid: uid,
            element_renderer: element_renderer,
            fired_events: Vec::new(),
        }
    }

    pub fn render(&self) -> iced::Element<'_, Message> {
        return self.element_renderer.render_element(self.root_uid).into();
    }

    pub fn emit_event(&mut self, event_uid: i32, event_data: EventResponse, is_dynamic: bool) {
        if is_dynamic {
            self.fired_events.push((event_uid, event_data.clone()));
        } else {
            let mut target: Option<i32> = None;
            let mut event_type: Option<String> = None;
            for event_listener in self.element_renderer.event_listeners.iter_mut() {
                if event_listener.event_uid == event_uid {
                    target = Some(event_listener.target);
                    event_type = Some(event_listener.event_type.clone());
                    self.fired_events
                        .push((event_listener.handler, event_data.clone()));
                }
            }
            if target.is_some() && event_type.is_some() {
                self.element_renderer.emit_internal_event(
                    target.unwrap(),
                    super::parser::XmlChangeEvent::EventFired(event_type.unwrap()),
                    false,
                );
            }
        }
    }
}
