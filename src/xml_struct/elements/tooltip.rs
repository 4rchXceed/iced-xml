// Copy-paste template
use crate::{
    dom::{events::DomInternalMessageType, query_builder::QueryResponse},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Tooltip {
    content: i32,
    tooltip: i32,
    position: iced::widget::tooltip::Position,
}

fn parse_position(pos: &str) -> iced::widget::tooltip::Position {
    return match pos {
        "top" => iced::widget::tooltip::Position::Top,
        "bottom" => iced::widget::tooltip::Position::Bottom,
        "left" => iced::widget::tooltip::Position::Left,
        "right" => iced::widget::tooltip::Position::Right,
        "cursor" => iced::widget::tooltip::Position::FollowCursor,
        _ => {
            println!(
                "Invalid <Tooltip> tip-pos value: {}. Defaulting to 'cursor'",
                pos
            );
            iced::widget::tooltip::Position::FollowCursor
        }
    };
}

impl ElementBase for Tooltip {
    fn new(
        xml_element: &XmlElement,
        renderer: &mut ElementRenderer,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        if xml_element.children.len() != 2 {
            return Err(ElementError::TooltipMustHaveTwoChildren(
                xml_element.clone(),
            ));
        }

        let mut tooltip = None;
        let mut content = None;

        for child in &xml_element.children {
            match child.tag.as_str() {
                "Content" => {
                    if child.children.len() != 1 {
                        return Err(ElementError::TooltipContentMustHaveOneChild(child.clone()));
                    }
                    content = Some(renderer.init_element_from_xml(&child.children[0], self_uid));
                }
                "Tip" => {
                    if child.children.len() != 1 {
                        return Err(ElementError::TooltipTipMustHaveOneChild(child.clone()));
                    }
                    tooltip = Some(renderer.init_element_from_xml(&child.children[0], self_uid));
                }
                _ => {
                    return Err(ElementError::TooltipChildrenMustBeContentAndTip(
                        child.clone(),
                    ));
                }
            }
        }
        if content.is_none() || tooltip.is_none() {
            panic!("<Tooltip> elements must have exactly one <Content /> and one <Tip /> child");
        }

        let mut position = iced::widget::tooltip::Position::FollowCursor;

        if xml_element.attributes.contains_key("tip-pos") {
            let pos = xml_element.attributes.get("tip-pos").unwrap();
            position = parse_position(pos);
        }

        return Ok(Self {
            content: content.unwrap(),
            tooltip: tooltip.unwrap(),
            position: position,
        });
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut tooltip = iced::widget::tooltip::Tooltip::new(
            renderer.render_element(self.content, datas.child_data.clone()),
            renderer.render_element(self.tooltip, datas.child_data),
            self.position,
        );

        tooltip = tooltip
            .delay(theme.tooltip_delay)
            .gap(theme.tooltip_gap)
            .padding(theme.tooltip_padding)
            .snap_within_viewport(theme.tooltip_no_overflow)
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

        return tooltip.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(name, newval) => match name.as_str() {
                "tip-pos" => {
                    self.position = parse_position(newval);
                    Some(ElementEventResponse::success())
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(name) => match name.as_str() {
                "tip-pos" => {
                    let pos_string = match self.position {
                        iced::widget::tooltip::Position::Top => "top".to_string(),
                        iced::widget::tooltip::Position::Bottom => "bottom".to_string(),
                        iced::widget::tooltip::Position::Left => "left".to_string(),
                        iced::widget::tooltip::Position::Right => "right".to_string(),
                        iced::widget::tooltip::Position::FollowCursor => "cursor".to_string(),
                    };
                    Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_str(pos_string),
                    ))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
