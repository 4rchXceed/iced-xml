use iced::{Background, widget::scrollable::AutoScroll};

// Copy-paste template
use crate::{
    dom::query::{EventResponse, QueryResponse},
    rs_utils::{HashableF32, ScrollState},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Scroll {
    child: i32,
    is_horizontal: bool,
}

impl ElementBase for Scroll {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        if xml_element.children.len() != 1 {
            panic!(
                "Scrollable MUST have one and only one child (currently has {})",
                xml_element.children.len()
            );
        }
        let mut is_horizontal = false;
        if xml_element.attributes.get("horizontal").is_some() {
            is_horizontal = true;
        }

        Self {
            child: renderer.init_element_from_xml(&xml_element.children[0], self_uid),
            is_horizontal,
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut scrollbar = datas.default_theme.clone();
        if datas.flag_themes.contains_key("scrollbar") {
            scrollbar = datas.flag_themes.get("scrollbar").unwrap().clone();
        }
        let mut scrollbar_scroller = datas.default_theme.clone();
        if datas.flag_themes.contains_key("scrollbar-scroller") {
            scrollbar_scroller = datas.flag_themes.get("scrollbar-scroller").unwrap().clone();
        }
        let mut gap = datas.default_theme.clone();
        if datas.flag_themes.contains_key("gap") {
            gap = datas.flag_themes.get("gap").unwrap().clone();
        }
        let mut autoscroll = datas.default_theme.clone();
        if datas.flag_themes.contains_key("autoscroll") {
            autoscroll = datas.flag_themes.get("autoscroll").unwrap().clone();
        }

        let mut scrollable =
            iced::widget::scrollable(renderer.render_element(self.child, datas.child_data.clone()));

        if theme.scroll_anchor.is_some() {
            scrollable = match theme.scroll_anchor.unwrap().as_str() {
                "top" => scrollable.anchor_top(),
                "bottom" => scrollable.anchor_bottom(),
                "left" => scrollable.anchor_left(),
                "right" => scrollable.anchor_right(),
                _ => scrollable,
            }
        }

        if self.is_horizontal {
            scrollable = scrollable.horizontal();
        }

        scrollable = scrollable
            .height(theme.height)
            .spacing(theme.spacing)
            .width(theme.width)
            .style(move |_, _| iced::widget::scrollable::Style {
                container: iced::widget::container::Style {
                    text_color: Some(theme.foreground_color),
                    background: Some(Background::Color(theme.background_color)),
                    border: iced::Border {
                        color: theme.border_color,
                        width: theme.border_width,
                        radius: theme.border_radius,
                    },
                    shadow: iced::Shadow {
                        color: theme.shadow_color,
                        offset: theme.shadow_offset,
                        blur_radius: theme.shadow_blur_radius,
                    },
                    snap: theme.snap,
                },
                vertical_rail: iced::widget::scrollable::Rail {
                    background: Some(Background::Color(scrollbar.background_color)),
                    border: iced::Border {
                        color: scrollbar.border_color,
                        width: scrollbar.border_width,
                        radius: scrollbar.border_radius,
                    },
                    scroller: iced::widget::scrollable::Scroller {
                        background: Background::Color(scrollbar_scroller.background_color),
                        border: iced::Border {
                            color: scrollbar_scroller.border_color,
                            width: scrollbar_scroller.border_width,
                            radius: scrollbar_scroller.border_radius,
                        },
                    },
                },
                horizontal_rail: iced::widget::scrollable::Rail {
                    background: Some(Background::Color(scrollbar.background_color)),
                    border: iced::Border {
                        color: scrollbar.border_color,
                        width: scrollbar.border_width,
                        radius: scrollbar.border_radius,
                    },
                    scroller: iced::widget::scrollable::Scroller {
                        background: Background::Color(scrollbar_scroller.background_color),
                        border: iced::Border {
                            color: scrollbar_scroller.border_color,
                            width: scrollbar_scroller.border_width,
                            radius: scrollbar_scroller.border_radius,
                        },
                    },
                },
                gap: Some(Background::Color(gap.background_color)),
                auto_scroll: AutoScroll {
                    background: Background::Color(autoscroll.background_color),
                    border: iced::Border {
                        color: autoscroll.border_color,
                        width: autoscroll.border_width,
                        radius: autoscroll.border_radius,
                    },
                    shadow: iced::Shadow {
                        color: autoscroll.shadow_color,
                        offset: autoscroll.shadow_offset,
                        blur_radius: autoscroll.shadow_blur_radius,
                    },
                    icon: autoscroll.foreground_color,
                },
            });
        let me = self_uid;
        // We don't need to change the state of the object when the event is triggered, so we can only create the event when the code is using the event.

        for event in events {
            match event.event_type.as_str() {
                "scroll" => {
                    scrollable = scrollable.on_scroll(move |v| {
                        let scroll_state = ScrollState {
                            scroll_height: HashableF32::new(v.bounds().height),
                            scroll_width: HashableF32::new(v.bounds().width),
                            scroll_left: HashableF32::new(v.relative_offset().x * v.bounds().width),
                            scroll_top: HashableF32::new(v.relative_offset().y * v.bounds().height),
                        };
                        let mut ev_response = EventResponse::new(me, "scroll".to_string());
                        ev_response.scrollable_scroll_state = Some(scroll_state);
                        return Message::DomEvent(event.event_uid, ev_response);
                    });
                }
                _ => (),
            }
        }

        return scrollable.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut query_response = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(key, value) => match key.as_str() {
                "horizontal" => {
                    self.is_horizontal = value == "true";
                    return Some((QueryResponse::new(true), vec![], vec![]));
                }
                _ => None,
            },
            XmlChangeEvent::GetProperty(key) => {
                if key == "horizontal" {
                    query_response.data_bool = Some(self.is_horizontal);
                    return Some((query_response, vec![], vec![]));
                }
                None
            }
            _ => None,
        }
    }
}
