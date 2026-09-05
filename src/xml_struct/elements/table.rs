use std::collections::HashMap;

// Copy-paste template
use crate::{
    dom::{events::DomInternalMessageType, query_builder::CustomElementEvent::SetTableData},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
            RenderChildDatas,
        },
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Table {
    datas: Vec<HashMap<String, String>>, // row data<column var id, value>
    columns: HashMap<i32, i32>,          // column name as child id -> template as child id
}

impl ElementBase for Table {
    fn new(
        xml_element: &XmlElement,
        renderer: &mut ElementRenderer,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        // If it supports children, initialize them here with renderer.init_element
        let mut columns = HashMap::new();
        for table_column in &xml_element.children {
            if table_column.tag == "TableColumn" {
                let mut column_name_elem_id = None;
                let mut column_template = None;
                for child in &table_column.children {
                    if child.tag == "ColumnName" {
                        if child.children.len() != 1 {
                            return Err(ElementError::ColumnNameElementMustHaveOneChild(
                                child.clone(),
                            ));
                        }
                        column_name_elem_id = Some(
                            renderer
                                .init_element_from_xml(child.children.get(0).unwrap(), self_uid),
                        );
                    } else if child.tag == "ColumnTemplate" {
                        if child.children.len() != 1 {
                            return Err(ElementError::ColumnTemplateElementMustHaveOneChild(
                                child.clone(),
                            ));
                        }
                        column_template = Some(
                            renderer
                                .init_element_from_xml(child.children.get(0).unwrap(), self_uid),
                        );
                    }
                }
                if column_name_elem_id.is_some() && column_template.is_some() {
                    columns.insert(column_name_elem_id.unwrap(), column_template.unwrap());
                } else {
                    return Err(ElementError::MissingChildInTableColumn(
                        table_column.clone(),
                    ));
                }
            } else {
                return Err(ElementError::TableCanOnlyHaveTableColumnChildren(
                    xml_element.clone(),
                    table_column.clone(),
                ));
            }
        }
        if columns.is_empty() {
            return Err(ElementError::TableHasNoChildren(xml_element.clone()));
        }

        return Ok(Self {
            columns: columns,
            datas: Vec::new(),
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

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::FireEvent(event) => match event {
                SetTableData(datas) => {
                    self.datas = datas.iter().map(|v| v.value().clone()).collect();
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            _ => None,
        }
    }
}
