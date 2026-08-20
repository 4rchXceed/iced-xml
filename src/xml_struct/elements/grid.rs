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

pub struct Grid {
    children: Vec<i32>,
}

impl ElementBase for Grid {
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
        let mut grid: iced::widget::grid::Grid<'a, Message> = iced::widget::grid::Grid::new();

        for child_uid in &self.children {
            grid = grid.push(renderer.render_element(child_uid.clone(), datas.child_data.clone()));
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

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            _ => None,
        }
    }
}
