use crate::{
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener},
        elements::element_base::ElementBase,
        parser::XmlElement,
    },
};

pub struct Col {
    children: Vec<i32>,
}

impl ElementBase for Col {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        let mut children: Vec<i32> = Vec::new();
        for child in &xml_element.children {
            children.push(renderer.init_element_from_xml(child, self_uid));
        }
        Self { children: children }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();

        for child in &self.children {
            container = container.push(renderer.render_element(*child, datas.child_data.clone()));
        }

        container = container
            .clip(theme.clip)
            .height(theme.height)
            .padding(theme.padding)
            .width(theme.width)
            .max_width(theme.max_width)
            .spacing(theme.spacing)
            .align_x(theme.align_x);
        if theme.wrap {
            return container.wrap().into();
        }

        return container.into();
    }
}
