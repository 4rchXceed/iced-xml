use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        elements::{
            ElementRenderer, EventListener, button::Button, center::Center, checkbox::Checkbox,
            container::Container, element_base::ElementBase, label::Label, row::Row,
        },
        parser::{XmlChangeEvent, XmlElement},
        theming::XmlTheme,
    },
};

pub enum AnyElement {
    Label(Label),
    Container(Container),
    Button(Button),
    Row(Row),
    Center(Center),
    Checkbox(Checkbox),
}

#[rustfmt::skip]
pub fn generate_element_from_tag(
    xml_element: &XmlElement,
    renderer: &mut ElementRenderer,
) -> Option<AnyElement> {
    return match xml_element.tag.as_str() {
        "Label" => Some(AnyElement::Label(Label::new(xml_element, renderer))),
        "Div" => Some(AnyElement::Container(Container::new(xml_element, renderer))),
        "Col" => Some(AnyElement::Container(Container::new(xml_element, renderer))),
        "Row" => Some(AnyElement::Row(Row::new(xml_element, renderer))),
        "Window" => Some(AnyElement::Container(Container::new(xml_element, renderer))), // Window works the same way as a container (FOR NOW), we'll use the same logic
        "Button" => Some(AnyElement::Button(Button::new(xml_element, renderer))),
        "Center" => Some(AnyElement::Center(Center::new(xml_element, renderer))),
        "Checkbox" => Some(AnyElement::Checkbox(Checkbox::new(xml_element, renderer))),
        _ => None,
    };
}

pub fn render_element<'a>(
    element: &'a AnyElement,
    renderer: &'a ElementRenderer,
    theme: &'a XmlTheme,
    events: Vec<&'a EventListener>,
    uid: i32,
) -> iced::Element<'a, Message> {
    return match element {
        AnyElement::Label(label) => label.render(renderer, theme, events, uid),
        AnyElement::Container(container) => container.render(renderer, theme, events, uid),
        AnyElement::Button(button) => button.render(renderer, theme, events, uid),
        AnyElement::Row(row) => row.render(renderer, theme, events, uid),
        AnyElement::Center(center) => center.render(renderer, theme, events, uid),
        AnyElement::Checkbox(checkbox) => checkbox.render(renderer, theme, events, uid),
    };
}

pub fn process_event_for_element<'a>(
    element: &'a mut AnyElement,
    event: XmlChangeEvent,
) -> Option<(QueryResponse, Vec<i32>)> {
    match element {
        AnyElement::Label(label) => label.process_event(&event),
        AnyElement::Container(container) => container.process_event(&event),
        AnyElement::Button(button) => button.process_event(&event),
        AnyElement::Row(row) => row.process_event(&event),
        AnyElement::Center(center) => center.process_event(&event),
        AnyElement::Checkbox(checkbox) => checkbox.process_event(&event),
    }
}
