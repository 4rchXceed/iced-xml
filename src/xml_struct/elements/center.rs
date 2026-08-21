use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::{element_base::ElementBase, label::Label, library::AnyElement},
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Center {
    children: Option<i32>,
    text: Option<String>,
    virtual_label: i32,
}

impl ElementBase for Center {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        if xml_element.children.len() != 1 && xml_element.text.trim().is_empty() {
            panic!("Center element must have exactly one child (or text inside)");
        }
        if xml_element.children.len() == 1 && !xml_element.text.trim().is_empty() {
            panic!("Center element must have either children or text, not both");
        }

        let virtual_label = renderer.init_element_virt(
            AnyElement::Label(Label::virt(xml_element.text.clone())),
            Some(xml_element.theme.clone()),
            self_uid,
        );

        if xml_element.text.trim().is_empty() {
            Self {
                children: Some(renderer.init_element_from_xml(&xml_element.children[0], self_uid)),
                text: None,
                virtual_label,
            }
        } else {
            Self {
                children: None,
                text: Some(xml_element.text.clone()),
                virtual_label,
            }
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut center;
        if self.children.is_some() {
            center = iced::widget::center(
                renderer.render_element(self.children.unwrap(), datas.child_data.clone()),
            );
        } else {
            center = iced::widget::center(iced::widget::text(self.text.clone().unwrap()));
        }
        center = center
            .clip(theme.clip)
            .max_height(theme.max_height)
            .max_width(theme.max_width)
            .width(theme.width)
            .height(theme.height)
            .style(move |_| iced::widget::container::Style {
                text_color: Some(theme.foreground_color),
                background: Some(theme.background),
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
            });
        if theme.center_use_align {
            center = center.align_y(theme.align_y).align_x(theme.align_x);
        }
        if theme.center_x {
            center = center.center_x(theme.width);
        }
        if theme.center_y {
            center = center.center_y(theme.height);
        }
        if theme.center_all {
            center = center.center(theme.width);
        }
        return center.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut result = QueryResponse::new(true);
        let mut elements_to_forward = Vec::new();
        match event {
            XmlChangeEvent::PropertyChange(key, newval) => {
                if key == "text" {
                    self.text = Some(newval.clone());
                    elements_to_forward.push(self.virtual_label);
                    Some((result, elements_to_forward, Vec::new()))
                } else {
                    None
                }
            }
            XmlChangeEvent::GetProperty(key) => {
                if key == "text" {
                    if self.text.is_some() {
                        result.data_str = self.text.clone();
                        Some((result, elements_to_forward, Vec::new()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
