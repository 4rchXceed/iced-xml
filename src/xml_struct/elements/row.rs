use crate::{
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener},
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Row {
    children: Vec<i32>,
}

impl ElementBase for Row {
    fn new(
        xml_element: &XmlElement,
        renderer: &mut ElementRenderer,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        let children: Vec<i32> = xml_element
            .children
            .iter()
            .map(|child| renderer.init_element_from_xml(child, self_uid))
            .collect();
        return Ok(Self { children: children });
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut container: iced::widget::Row<'a, Message> = iced::widget::Row::new();

        for child in &self.children {
            container = container.push(renderer.render_element(*child, datas.child_data.clone()));
        }

        container = container
            .clip(theme.clip)
            .height(theme.height)
            .padding(theme.padding)
            .width(theme.width)
            .spacing(theme.spacing)
            .align_y(theme.align_y);
        if theme.wrap {
            return container.wrap().into();
        }

        return container.into();
    }
}
