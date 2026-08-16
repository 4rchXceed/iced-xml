use iced::Border;

// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    rs_utils::HashableF32,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Progress {
    progress: f32,
    min: f32,
    max: f32,
    vertical: bool,
}

impl ElementBase for Progress {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        let min_op = xml_element.attributes.get("min");
        let max_op = xml_element.attributes.get("max");
        let mut vertical = false;
        if min_op.is_none() || max_op.is_none() {
            panic!("Progress element must have 'min' and 'max' attributes");
        }
        let progress = xml_element
            .attributes
            .get("value")
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);
        if xml_element.attributes.get("vertical").is_some() {
            vertical = true;
        }
        let min = min_op.unwrap().parse::<f32>().unwrap_or(0.0);
        let max = max_op.unwrap().parse::<f32>().unwrap_or(100.0);

        Self {
            progress,
            min,
            max,
            vertical,
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut progress =
            iced::widget::progress_bar::ProgressBar::new(self.min..=self.max, self.progress);

        progress = progress
            .length(theme.width)
            .style(move |_| iced::widget::progress_bar::Style {
                background: iced::Background::Color(theme.background_color),
                bar: iced::Background::Color(theme.foreground_color),
                border: Border {
                    radius: theme.border_radius,
                    width: theme.border_width,
                    color: theme.border_color,
                },
            });

        if theme.progress_height.is_some() {
            progress = progress.girth(theme.progress_height.unwrap());
        }

        if self.vertical {
            progress = progress.vertical();
        }

        return progress.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut qr = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(name, new_val) => match name.as_str() {
                "value" => {
                    self.progress = new_val.parse::<f32>().unwrap_or(self.progress);
                    Some((qr, Vec::new(), Vec::new()))
                }
                "min" => {
                    self.min = new_val.parse::<f32>().unwrap_or(self.min);
                    Some((qr, Vec::new(), Vec::new()))
                }
                "max" => {
                    self.max = new_val.parse::<f32>().unwrap_or(self.max);
                    Some((qr, Vec::new(), Vec::new()))
                }
                _ => None,
            },
            XmlChangeEvent::GetProperty(name) => match name.as_str() {
                "value" => {
                    qr.data_float = Some(HashableF32::new(self.progress));
                    Some((qr, Vec::new(), Vec::new()))
                }
                "min" => {
                    qr.data_float = Some(HashableF32::new(self.min));
                    Some((qr, Vec::new(), Vec::new()))
                }
                "max" => {
                    qr.data_float = Some(HashableF32::new(self.max));
                    Some((qr, Vec::new(), Vec::new()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
