use std::time::Duration;

use crate::{
    dom::{
        types::{DomInternalMessageType, EventListenerTypes},
        types::{EventResponse, QueryResponse},
    },
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener},
    },
    rs_utils::{HashableF32, Vector2},
    xml_engine::Message,
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};

pub struct Trigger {
    child: Option<i32>,
    anticipated_pixels: f32,
    time_trigger: u64,
}

impl ElementBase for Trigger {
    fn new(
        xml_element: Box<XmlElement>,
        renderer: &mut ElementMappings,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        if let Some(first_child) = xml_element.children.get(0) {
            let child = Some(renderer.init_element_from_xml(first_child.clone(), self_uid));

            let mut anticipated_pixels: f32 = 0.0;
            let mut time_to_trigger: u64 = 0;

            if let Some(attr) = xml_element.attributes.get("anticipated_pixels") {
                anticipated_pixels = attr.parse::<f32>().unwrap_or(0.0);
            }
            if let Some(attr) = xml_element.attributes.get("time_to_trigger") {
                time_to_trigger = attr.parse::<u64>().unwrap_or(0);
            }

            return Ok(Self {
                child: child,
                anticipated_pixels: anticipated_pixels,
                time_trigger: time_to_trigger,
            });
        } else {
            return Err(ElementError::TriggerElementHasMoreThanOneChild(
                xml_element.clone(),
            ));
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementsManager,
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
            match event.event_type {
                EventListenerTypes::Hidden => {
                    trigger = trigger.on_hide(Message::DomEvent(
                        Some(event.event_uid),

                        EventResponse::new(me, event.event_type.clone()),
                    ))
                }
                EventListenerTypes::Shown => {
                    trigger = trigger.on_show(move |size| {
                        let mut ev_res = EventResponse::new(me, event.event_type.clone());
                        
                        ev_res.data_vector = Some(Vector2 {
                            x: HashableF32::new(size.width),
                            y: HashableF32::new(size.height),
                        });

                        Message::DomEvent(Some(event.event_uid), ev_res)
                    })
                }
                EventListenerTypes::Resize => {
                    trigger = trigger.on_resize(move |size| {
                        let mut ev_res = EventResponse::new(me, event.event_type.clone());
                        
                        ev_res.data_vector = Some(Vector2 {
                            x: HashableF32::new(size.width),
                            y: HashableF32::new(size.height),
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
                    if let Ok(parsed_val) = val.parse::<f32>() {
                        self.anticipated_pixels = parsed_val;

                        return Some(ElementEventResponse::success());
                    } else {
                        return Some(ElementEventResponse::new(QueryResponse::fail(&format!(
                            "Failed to parse anticipated_pixels value: {}",
                            val
                        ))));
                    }
                }
                "time_trigger" => {
                    if let Ok(parsed_val) = val.parse::<u64>() {
                        self.time_trigger = parsed_val;

                        return Some(ElementEventResponse::success());
                    } else {
                        return Some(ElementEventResponse::new(QueryResponse::fail(&format!(
                            "Failed to parse time_trigger value: {}",
                            val
                        ))));
                    }
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
