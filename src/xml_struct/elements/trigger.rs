use std::time::Duration;

use crate::{
    dom::{
        events::DomInternalMessageType,
        query::{EventResponse, QueryResponse},
    },
    rs_utils::{HashableF32, VectorWH},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::element_base::ElementBase,
        parser::XmlElement,
    },
};

pub struct Trigger {
    child: Option<i32>,
    anticipated_pixels: f32,
    time_trigger: u64,
}

impl ElementBase for Trigger {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        if xml_element.children.len() > 1 {
            panic!("Trigger element can only have zero or one child");
        }
        let mut child: Option<i32> = None;
        if xml_element.children.len() == 1 {
            child = Some(renderer.init_element_from_xml(&xml_element.children[0], self_uid));
        }
        let mut anticipated_pixels: f32 = 0.0;
        let mut time_to_trigger: u64 = 0;
        if xml_element.attributes.contains_key("anticipated_pixels") {
            anticipated_pixels = xml_element
                .attributes
                .get("anticipated_pixels")
                .unwrap()
                .parse::<f32>()
                .unwrap();
        }
        if xml_element.attributes.contains_key("time_trigger") {
            time_to_trigger = xml_element
                .attributes
                .get("time_to_trigger")
                .unwrap()
                .parse::<u64>()
                .unwrap();
        }

        Self {
            child: child,
            anticipated_pixels: anticipated_pixels,
            time_trigger: time_to_trigger,
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let mut child_element = iced::widget::Column::new();
        if let Some(child_uid) = self.child {
            child_element =
                child_element.push(renderer.render_element(child_uid, datas.child_data.clone()));
        }
        let mut trigger = iced::widget::sensor::Sensor::new(child_element);

        trigger = trigger
            .anticipate(self.anticipated_pixels)
            .delay(Duration::from_millis(self.time_trigger as u64));

        let me = self_uid;

        for event in events {
            match event.event_type.as_str() {
                "hidden" => {
                    trigger = trigger.on_hide(Message::DomEvent(
                        Some(event.event_uid),
                        EventResponse::new(me, String::from("hidden")),
                    ))
                }
                "shown" => {
                    trigger = trigger.on_show(move |size| {
                        let mut ev_res = EventResponse::new(me, String::from("shown"));
                        ev_res.data_vectorwh = Some(VectorWH {
                            width: HashableF32::new(size.width),
                            height: HashableF32::new(size.height),
                        });
                        Message::DomEvent(Some(event.event_uid), ev_res)
                    })
                }
                "resize" => {
                    trigger = trigger.on_resize(move |size| {
                        let mut ev_res = EventResponse::new(me, String::from("resize"));
                        ev_res.data_vectorwh = Some(VectorWH {
                            width: HashableF32::new(size.width),
                            height: HashableF32::new(size.height),
                        });
                        Message::DomEvent(Some(event.event_uid), ev_res)
                    })
                }
                _ => (),
            }
        }

        return trigger.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(key, val) => match key.as_str() {
                "anticipated_pixels" => {
                    self.anticipated_pixels = val.parse::<f32>().unwrap();
                    return Some(ElementEventResponse::success());
                }
                "time_trigger" => {
                    self.time_trigger = val.parse::<u64>().unwrap();
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(name) => match name.as_str() {
                "anticipated_pixels" => {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_float(self.anticipated_pixels),
                    ));
                }
                "time_trigger" => {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_float(self.time_trigger as f32),
                    ));
                }
                _ => None,
            },
            _ => None,
        }
    }
}
