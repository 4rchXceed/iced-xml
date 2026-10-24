// Copy-paste template
use crate::{
    elements_manager::{elements_manager::ElementsManager, mappings::ElementMappings, utils::{ElementExtraData, EventListener}}, xml_engine::Message, xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    }
};

pub struct Grid {
    children: Vec<i32>,
}

impl ElementBase for Grid {
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
        let mut grid: iced::widget::grid::Grid<'a, Message> = iced::widget::grid::Grid::new();

        for child_uid in &self.children {
            grid = grid.push(manager.render_element(child_uid.clone(), datas.child_data.clone()));
        }

        grid = grid
            .columns(theme.grid_columns)
            .height(theme.height)
            .spacing(theme.spacing);

        if let Some(grid_width) = theme.grid_width {
            grid = grid.width(grid_width);
        }

        if let Some(responsive_width) = theme.grid_responsive_width {
            grid = grid.fluid(responsive_width);
        }

        return grid.into();
    }
}
