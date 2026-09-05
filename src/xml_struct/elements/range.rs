use std::ops::RangeInclusive;

// Copy-paste template
use crate::{
    dom::{
        events::DomInternalMessageType,
        query_builder::{EventResponse, QueryResponse},
    },
    rs_utils::HashableF32,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::element_base::ElementBase,
        parser::XmlElement,
        theming::XmlTheme,
    },
};

pub fn parse_property(val: String) -> (QueryResponse, Option<f32>) {
    let new_value = val.parse::<f32>();
    if new_value.is_err() {
        return (
            QueryResponse::fail(format!("Property: {} isn't a valid float", val).as_str()),
            None,
        );
    }
    return (QueryResponse::success(), Some(new_value.unwrap()));
}

pub struct Range {
    min: f32,
    max: f32,
    value: f32,
    default: Option<f32>,
    step: f32,
    second_step: Option<f32>,
    vertical: bool,
}

impl Range {
    fn horizontal<'a>(
        &self,
        theme: XmlTheme,
        on_input: impl Fn(f32) -> Message + 'a,
        on_release: Option<Message>,
        style: iced::widget::slider::Style,
    ) -> iced::Element<'a, Message> {
        let mut slider = iced::widget::Slider::new(
            RangeInclusive::new(self.min, self.max),
            self.value,
            on_input,
        );
        slider = slider
            .height(theme.slider_height)
            .width(theme.width)
            .step(self.step)
            .style(move |_, _| style);

        if on_release.is_some() {
            slider = slider.on_release(on_release.unwrap());
        }

        if self.second_step.is_some() {
            slider = slider.shift_step(self.second_step.unwrap());
        }

        if self.default.is_some() {
            slider = slider.default(self.default.unwrap());
        }

        return slider.into();
    }

    fn vertical<'a>(
        &self,
        theme: XmlTheme,
        on_input: impl Fn(f32) -> Message + 'a,
        on_release: Option<Message>,
        style: iced::widget::slider::Style,
    ) -> iced::Element<'a, Message> {
        let mut slider = iced::widget::VerticalSlider::new(
            RangeInclusive::new(self.min, self.max),
            self.value,
            on_input,
        );
        slider = slider
            .height(theme.height)
            .width(theme.vertical_slider_width)
            .step(self.step)
            .style(move |_, _| style);

        if on_release.is_some() {
            slider = slider.on_release(on_release.unwrap());
        }

        if self.second_step.is_some() {
            slider = slider.shift_step(self.second_step.unwrap());
        }

        if self.default.is_some() {
            slider = slider.default(self.default.unwrap());
        }

        return slider.into();
    }
}

impl ElementBase for Range {
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
            vertical: xml_element.attributes.contains_key("vertical"),
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

        let mut id = None;
        for event in &events {
            match event.event_type.as_str() {
                "input" => {
                    id = Some(event.event_uid);
                }
                _ => (),
            }
        }
        let on_input = move |v| {
            let mut event_response = EventResponse::new(self_uid, String::from("input"));
            event_response.data_float = Some(HashableF32::new(v));
            return Message::DomEvent(id, event_response);
        };

        let mut on_release_message = None;

        for event in events {
            match event.event_type.as_str() {
                "release" => {
                    on_release_message = Some(Message::DomEvent(
                        Some(event.event_uid),
                        EventResponse::new(self_uid, String::from("release")),
                    ));
                }
                _ => (),
            }
        }
        let style = iced::widget::slider::Style {
            rail: iced::widget::slider::Rail {
                backgrounds: (theme.background, theme.foreground_element),
                width: theme.slider_rail_width,
                border: iced::Border {
                    color: theme.border_color,
                    width: theme.border_width,
                    radius: theme.border_radius,
                },
            },
            handle: iced::widget::slider::Handle {
                shape: handle_theme.slider_handle_shape.clone(),
                background: handle_theme.background,
                border_width: handle_theme.border_width,
                border_color: handle_theme.border_color,
            },
        };
        if self.vertical {
            return self.vertical(theme, on_input, on_release_message, style);
        } else {
            return self.horizontal(theme, on_input, on_release_message, style);
        }
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        return match event {
            DomInternalMessageType::PropertyChange(key, value) => {
                return match key.as_str() {
                    "value" => {
                        let r = parse_property(value.clone());
                        self.value = r.1.unwrap_or(self.value);
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "min" => {
                        let r = parse_property(value.clone());
                        self.min = r.1.unwrap_or(self.min);
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "max" => {
                        let r = parse_property(value.clone());
                        self.max = r.1.unwrap_or(self.max);
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "default" => {
                        let r = parse_property(value.clone());
                        self.default = r.1;
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "step" => {
                        let r = parse_property(value.clone());
                        self.step = r.1.unwrap_or(self.step);
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "second-step" => {
                        let r = parse_property(value.clone());
                        self.second_step = r.1;
                        return Some(ElementEventResponse::new(r.0));
                    }
                    "vertical" => {
                        self.vertical = value == "true";
                        return Some(ElementEventResponse::success());
                    }
                    _ => None,
                };
            }
            DomInternalMessageType::GetProperty(key) => {
                match key.as_str() {
                    "value" => {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_float(self.value),
                        ));
                    }
                    "min" => {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_float(self.min),
                        ));
                    }
                    "max" => {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_float(self.max),
                        ));
                    }
                    "default" => {
                        if self.default.is_some() {
                            return Some(ElementEventResponse::new(
                                QueryResponse::success().with_data_float(self.default.unwrap()),
                            ));
                        } else {
                            return Some(ElementEventResponse::new(QueryResponse::success())); // We don't fail if the default is not set, we just return success with no data
                        }
                    }
                    "step" => {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_float(self.step),
                        ));
                    }
                    "second-step" => {
                        if self.second_step.is_some() {
                            return Some(ElementEventResponse::new(
                                QueryResponse::success().with_data_float(self.second_step.unwrap()),
                            ));
                        } else {
                            return Some(ElementEventResponse::new(QueryResponse::success())); // Same here
                        }
                    }
                    "vertical" => {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_bool(self.vertical),
                        ));
                    }
                    _ => None,
                }
            }
            _ => None,
        };
    }

    fn event_callback(
        &mut self,
        event_type: &String,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type.as_str() {
            "input" => {
                self.value = event_response
                    .data_float
                    .clone()
                    .unwrap_or(HashableF32::new(self.value))
                    .value();
                return Some(ElementEventResponse::success());
            }
            _ => None,
        }
    }
}
