use crate::{
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementExtraData, EventListener},
    },
    xml_engine::Message,
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};

pub struct Col {
    children: Vec<i32>,
}

impl ElementBase for Col {
    fn new(
        xml_element: Box<XmlElement>,
        mappings: &mut ElementMappings,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        let children: Vec<i32> = xml_element
            .children
            .iter()
            .map(|child| mappings.init_element_from_xml(child.clone(), self_uid))
            .collect();

        return Ok(Self { children: children });
    }

    fn render<'a>(
        &self,
        manager: &'a ElementsManager,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();

        for child in &self.children {
            container = container.push(manager.render_element(*child, datas.child_data.clone()));
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
