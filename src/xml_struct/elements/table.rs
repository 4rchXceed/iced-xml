use std::collections::HashMap;

// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementExtraData, ElementRenderer, EventListener, RenderChildDatas, RendererEvent,
        },
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Table {
    datas: Vec<HashMap<String, String>>, // row data<column var id, value>
    columns: HashMap<i32, i32>,          // column name as child id -> template as child id
}

impl ElementBase for Table {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, _: i32) -> Self {
        // If it supports children, initialize them here with renderer.init_element
        let mut columns = HashMap::new();
        for table_column in &xml_element.children {
            if table_column.tag == "TableColumn" {
                let mut column_name_elem_id = None;
                let mut column_template = None;
                for child in &table_column.children {
                    if child.tag == "ColumnName" {
                        if child.children.len() != 1 {
                            panic!(
                                "<ColumnName> must have only one child element, which will be used as the column name"
                            );
                        }
                        column_name_elem_id =
                            Some(renderer.init_element_from_xml(child.children.get(0).unwrap()));
                    } else if child.tag == "ColumnTemplate" {
                        if child.children.len() != 1 {
                            panic!(
                                "<ColumnTemplate> must have only one child element, which will be used as the column template"
                            );
                        }
                        column_template =
                            Some(renderer.init_element_from_xml(child.children.get(0).unwrap()));
                    }
                }
                if column_name_elem_id.is_some() && column_template.is_some() {
                    columns.insert(column_name_elem_id.unwrap(), column_template.unwrap());
                } else {
                    panic!("TableColumn must have both <ColumnName> and <ColumnTemplate> children");
                }
            } else {
                panic!("Table element can only have <TableColumn> children");
            }
        }
        if columns.is_empty() {
            panic!("Table element must have at least one <TableColumn> child");
        }

        Self {
            columns: columns,
            datas: Vec::new(),
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
        let mut column_theme = theme.clone();
        if datas.flag_themes.contains_key("table-column") {
            column_theme = datas.flag_themes.get("table-column").unwrap().clone();
        }
        let mut columns = Vec::new();

        for (column_name_elem_id, column_template_elem_id) in &self.columns {
            let column_name_elem =
                renderer.render_element(*column_name_elem_id, datas.child_data.clone());

            columns.push(
                iced::widget::table::column(column_name_elem, |ev: HashMap<String, String>| {
                    return renderer.render_element(
                        *column_template_elem_id,
                        Some(RenderChildDatas {
                            table_datas: Some(ev.clone()),
                            ..RenderChildDatas::default()
                        }),
                    );
                })
                .align_x(column_theme.align_x)
                .align_y(column_theme.align_y)
                .width(column_theme.width),
            );
        }

        let mut table = iced::widget::table::Table::new(columns, self.datas.clone());

        table = table
            .padding_x(theme.table_padding.0)
            .padding_y(theme.table_padding.1)
            .separator_x(theme.table_separator_width.0)
            .separator_y(theme.table_separator_width.1)
            .width(theme.width);

        return table.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            XmlChangeEvent::EmittedEvent(event_name, datas) => match event_name.as_str() {
                "set-data" => {
                    if datas.data_tabledata.is_some() {
                        self.datas = datas
                            .data_tabledata
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|v| v.value().clone())
                            .collect();
                        return Some((QueryResponse::new(true), vec![], vec![]));
                    } else {
                        return None;
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
