use iced::Shadow;

// Copy-paste template
use crate::{
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener},
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct FloatingElement {
    child: i32,
}

impl ElementBase for FloatingElement {
    fn new(
        xml_element: &XmlElement,
        renderer: &mut ElementRenderer,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        if xml_element.children.len() != 1 {
            return Err(ElementError::FloatingElementMustHaveOneChild(
                xml_element.clone(),
            ));
        }
        let child = renderer.init_element_from_xml(&xml_element.children[0], self_uid);

        return Ok(Self { child: child });
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut float_element: iced::widget::float::Float<'a, Message> =
            iced::widget::float::Float::new(
                renderer.render_element(self.child, datas.child_data.clone()),
            );

        float_element =
            float_element
                .scale(theme.scale)
                .style(move |_| iced::widget::float::Style {
                    shadow: Shadow {
                        blur_radius: theme.shadow_blur_radius,
                        color: theme.shadow_color,
                        offset: theme.shadow_offset,
                    },
                    shadow_border_radius: theme.border_radius,
                });
        // TODO: What does .translate do?

        return float_element.into();
    }
}
