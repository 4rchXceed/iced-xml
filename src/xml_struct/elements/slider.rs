use std::ops::RangeInclusive;

use iced::Background;

// Copy-paste template
use crate::{
    dom::query::{EventResponse, QueryResponse},
    rs_utils::HashableF32,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub fn parse_property(val: String) -> (QueryResponse, Option<f32>) {
    let new_value = val.parse::<f32>();
    if new_value.is_err() {
        return (QueryResponse::new(false), None);
    }
    return (QueryResponse::new(true), Some(new_value.unwrap()));
}

pub struct Slider {
    min: f32,
    max: f32,
    value: f32,
    default: Option<f32>,
    step: f32,
    second_step: Option<f32>,
}

impl ElementBase for Slider {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        let mut min = 0.0;
        let mut max = 1.0;
        let mut value = 0.0;
        let mut default = None;
        let mut shift_step = None;
        let mut step = 0.1;
        if xml_element.attributes.contains_key("min") {
            min = xml_element.attributes["min"].parse::<f32>().unwrap_or(0.0);
        }
        if xml_element.attributes.contains_key("max") {
            max = xml_element.attributes["max"].parse::<f32>().unwrap_or(1.0);
        }
        if xml_element.attributes.contains_key("value") {
            value = xml_element.attributes["value"]
                .parse::<f32>()
                .unwrap_or(0.0);
        }
        if xml_element.attributes.contains_key("default") {
            default = Some(
                xml_element.attributes["default"]
                    .parse::<f32>()
                    .unwrap_or(0.0),
            );
        }
        if xml_element.attributes.contains_key("step") {
            step = xml_element.attributes["step"].parse::<f32>().unwrap_or(0.1);
        }
        if xml_element.attributes.contains_key("second-step") {
            shift_step = Some(
                xml_element.attributes["second-step"]
                    .parse::<f32>()
                    .unwrap_or(0.01),
            );
        }

        Self {
            min: min,
            max: max,
            value: value,
            default: default,
            step: step,
            second_step: shift_step,
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut handle_theme = theme.clone();
        if datas.flag_themes.contains_key("handle") {
            handle_theme = datas.flag_themes["handle"].clone();
        }

        let mut id = -1;
        for event in &events {
            match event.event_type.as_str() {
                "input" => {
                    id = event.event_uid;
                }
                _ => (),
            }
        }

        let mut slider = iced::widget::Slider::new(
            RangeInclusive::new(self.min, self.max),
            self.value,
            move |v| {
                let mut event_response = EventResponse::new(self_uid, String::from("input"));
                event_response.data_float = Some(HashableF32::new(v));
                return Message::DomEvent(id, event_response);
            },
        );

        slider = slider
            .height(theme.slider_height)
            .width(theme.width)
            .step(self.step)
            .style(move |_, _| iced::widget::slider::Style {
                rail: iced::widget::slider::Rail {
                    backgrounds: (
                        Background::Color(theme.background_color),
                        Background::Color(theme.foreground_color),
                    ),
                    width: theme.slider_rail_width,
                    border: iced::Border {
                        color: theme.border_color,
                        width: theme.border_width,
                        radius: theme.border_radius,
                    },
                },
                handle: iced::widget::slider::Handle {
                    shape: handle_theme.slider_handle_shape.clone(),
                    background: Background::Color(handle_theme.background_color),
                    border_width: handle_theme.border_width,
                    border_color: handle_theme.border_color,
                },
            });

        for event in events {
            match event.event_type.as_str() {
                "release" => {
                    slider = slider.on_release(Message::DomEvent(
                        event.event_uid,
                        EventResponse::new(self_uid, String::from("release")),
                    ));
                }
                _ => (),
            }
        }

        if self.second_step.is_some() {
            slider = slider.shift_step(self.second_step.unwrap());
        }

        if self.default.is_some() {
            slider = slider.default(self.default.unwrap());
        }

        return slider.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        return match event {
            XmlChangeEvent::EventFired(ev_type, value) => {
                if ev_type == "input" {
                    self.value = value
                        .data_float
                        .clone()
                        .unwrap_or(HashableF32::new(self.value))
                        .value();
                    return Some((QueryResponse::new(true), Vec::new(), Vec::new()));
                }
                None
            }
            XmlChangeEvent::PropertyChange(key, value) => {
                return match key.as_str() {
                    "value" => {
                        let r = parse_property(value.clone());
                        self.value = r.1.unwrap_or(self.value);
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    "min" => {
                        let r = parse_property(value.clone());
                        self.min = r.1.unwrap_or(self.min);
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    "max" => {
                        let r = parse_property(value.clone());
                        self.max = r.1.unwrap_or(self.max);
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    "default" => {
                        let r = parse_property(value.clone());
                        self.default = r.1;
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    "step" => {
                        let r = parse_property(value.clone());
                        self.step = r.1.unwrap_or(self.step);
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    "second-step" => {
                        let r = parse_property(value.clone());
                        self.second_step = r.1;
                        return Some((r.0, Vec::new(), Vec::new()));
                    }
                    _ => None,
                };
            }
            XmlChangeEvent::GetProperty(key) => {
                let mut result = QueryResponse::new(true);
                match key.as_str() {
                    "value" => {
                        result.data_float = Some(HashableF32::new(self.value));
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    "min" => {
                        result.data_float = Some(HashableF32::new(self.min));
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    "max" => {
                        result.data_float = Some(HashableF32::new(self.max));
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    "default" => {
                        if self.default.is_some() {
                            result.data_float = Some(HashableF32::new(self.default.unwrap()));
                        }
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    "step" => {
                        result.data_float = Some(HashableF32::new(self.step));
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    "second-step" => {
                        if self.second_step.is_some() {
                            result.data_float = Some(HashableF32::new(self.second_step.unwrap()));
                        }
                        return Some((result, Vec::new(), Vec::new()));
                    }
                    _ => None,
                }
            }
            _ => None,
        };
    }
}
