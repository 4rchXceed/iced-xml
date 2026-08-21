use iced::{Border, Shadow};

// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Container {
    child: i32,
}

impl ElementBase for Container {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        if xml_element.children.len() != 1 {
            panic!(
                "Container MUST have one and only one child (currently has {})",
                xml_element.children.len()
            );
        }

        Self {
            child: renderer.init_element_from_xml(&xml_element.children[0], self_uid),
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

        let mut container: iced::widget::Container<'a, Message> = iced::widget::Container::new(
            renderer.render_element(self.child, datas.child_data.clone()),
        );

        container = container
            .align_x(theme.align_x)
            .align_y(theme.align_y)
            .clip(theme.clip)
            .height(theme.height)
            .max_height(theme.max_height)
            .max_width(theme.max_width)
            .padding(theme.padding)
            .width(theme.width)
            .style(move |_| iced::widget::container::Style {
                background: Some(theme.background),
                shadow: Shadow {
                    color: theme.shadow_color,
                    offset: theme.shadow_offset,
                    blur_radius: theme.shadow_blur_radius,
                },
                border: Border {
                    color: theme.border_color,
                    radius: theme.border_radius,
                    width: theme.border_width,
                },
                snap: theme.snap,
                text_color: Some(theme.foreground_color),
            });
        if theme.center_all {
            container = container.center(theme.width); // FOR_DOC: This will only use the width
        }
        if theme.center_x {
            container = container.center_x(theme.width);
        }
        if theme.center_y {
            container = container.center_y(theme.height);
        }
        return container.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            _ => None,
        }
    }
}
