use crate::{
    dom::{types::DomInternalMessageType, types::QueryResponse},
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener},
    },
    xml_engine::Message,
    xml_struct::{
        elements::{
            element_base::ElementBase,
            label::Label,
            library::{AnyElement, ElementError},
        },
        xml_element::XmlElement,
    },
};

pub enum Content {
    Children(i32),
    Text(String, i32),
}

pub struct Center {
    content: Content,
}

impl ElementBase for Center {
    fn new(
        xml_element: Box<XmlElement>,
        mappings: &mut ElementMappings,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        if xml_element.children.len() != 1 && xml_element.text.trim().is_empty() {
            return Err(ElementError::CenterElementNotOneChildren(
                xml_element.clone(),
            ));
        }

        if xml_element.children.len() == 1 && !xml_element.text.trim().is_empty() {
            return Err(ElementError::CenterElementHasBothChildrenAndText(
                xml_element.clone(),
            ));
        }

        if xml_element.text.trim().is_empty() {
            let child_id =
                mappings.init_element_from_xml(xml_element.children[0].clone(), self_uid);

            return Ok(Self {
                content: Content::Children(child_id),
            });
        } else {
            let virtual_label = mappings.init_element_virt(
                AnyElement::Label(Label::virt(xml_element.text.clone())),
                Some(xml_element.theme.clone()),
                self_uid,
            );

            return Ok(Self {
                content: Content::Text(xml_element.text.clone(), virtual_label),
            });
        }
    }

    fn render<'a>(
        &self,
        manager: &'a ElementsManager,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();

        let child_id = match &self.content {
            Content::Children(child) => *child,
            Content::Text(_, virtual_label) => *virtual_label,
        };

        let mut center =
            iced::widget::center(manager.render_element(child_id, datas.child_data.clone()));

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

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(key, newval) => {
                if key == "text" {
                    match &mut self.content {
                        Content::Text(text, virtual_label) => {
                            *text = newval.clone();

                            Some(
                                ElementEventResponse::success()
                                    .with_forward_to(vec![*virtual_label]),
                            )
                        }
                        _ => Some(ElementEventResponse::new(QueryResponse::fail(
                            "Wanted to change text, but <Center /> has a child, not text, as content",
                        ))),
                    }
                } else {
                    None
                }
            }
            DomInternalMessageType::GetProperty(key) => {
                if key == "text" {
                    match &self.content {
                        Content::Text(text, _) => Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_str(text.clone()),
                        )),
                        _ => Some(ElementEventResponse::new(QueryResponse::fail(
                            "Wanted to get text, but <Center /> has a child, not text, as content",
                        ))),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
