use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::{
            button::Button, center::Center, checkbox::Checkbox, col::Col, container::Container,
            element_base::ElementBase, float::FloatingElement, grid::Grid, label::Label,
            progress::Progress, radio::RadioButton, row::Row, scrollable::Scroll, select::Select,
            trigger::Trigger, window_system::WindowSystem,
        },
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub enum AnyElement {
    Label(Label),
    Col(Col),
    Button(Button),
    Row(Row),
    Center(Center),
    Checkbox(Checkbox),
    Select(Select),
    Container(Container),
    FloatingElement(FloatingElement),
    Grid(Grid),
    WindowSystem(WindowSystem),
    Radio(RadioButton),
    Scroll(Scroll),
    Progress(Progress),
    Trigger(Trigger),
}

#[rustfmt::skip]
pub fn generate_element_from_tag(
    xml_element: &XmlElement,
    renderer: &mut ElementRenderer,
    self_uid: i32
) -> Option<AnyElement> {
    return match xml_element.tag.as_str() {
        "Label" => Some(AnyElement::Label(Label::new(xml_element, renderer, self_uid))),
        "Div" => Some(AnyElement::Container(Container::new(xml_element, renderer, self_uid))),
        "Col" => Some(AnyElement::Col(Col::new(xml_element, renderer, self_uid))),
        "Row" => Some(AnyElement::Row(Row::new(xml_element, renderer, self_uid))),
        "Window" => Some(AnyElement::Col(Col::new(xml_element, renderer, self_uid))), // Window works the same way as a container (FOR NOW), we'll use the same logic
        "WindowContent" => Some(AnyElement::Col(Col::new(xml_element, renderer, self_uid))), // Same for WindowContent
        "CloseButton" => Some(AnyElement::Col(Col::new(xml_element, renderer, self_uid))), // Same for WindowContent
        "MaximizeButton" => Some(AnyElement::Col(Col::new(xml_element, renderer, self_uid))), // Same for WindowContent
        "Content" => Some(AnyElement::Container(Container::new(xml_element, renderer, self_uid))), // Same for WindowContent
        "Button" => Some(AnyElement::Button(Button::new(xml_element, renderer, self_uid))),
        "Center" => Some(AnyElement::Center(Center::new(xml_element, renderer, self_uid))),
        "Radio" => Some(AnyElement::Radio(RadioButton::new(xml_element, renderer, self_uid))),
        "Checkbox" => Some(AnyElement::Checkbox(Checkbox::new(xml_element, renderer, self_uid))),
        "Select" => Some(AnyElement::Select(Select::new(xml_element, renderer, self_uid))),
        "Float" => Some(AnyElement::FloatingElement(FloatingElement::new(xml_element, renderer, self_uid))),
        "Grid" => Some(AnyElement::Grid(Grid::new(xml_element, renderer, self_uid))),
        "WindowSystem" => Some(AnyElement::WindowSystem(WindowSystem::new(xml_element, renderer, self_uid))),
        "Scroll" => Some(AnyElement::Scroll(Scroll::new(xml_element, renderer, self_uid))),
        "Progress" => Some(AnyElement::Progress(Progress::new(xml_element, renderer, self_uid))),
        "Trigger" => Some(AnyElement::Trigger(Trigger::new(xml_element, renderer, self_uid))),
        _ => None,
    };
}

pub fn render_element<'a>(
    element: &'a AnyElement,
    renderer: &'a ElementRenderer,
    datas: &'a ElementExtraData,
    events: Vec<&'a EventListener>,
    uid: i32,
) -> iced::Element<'a, Message> {
    return match element {
        AnyElement::Label(label) => label.render(renderer, datas, events, uid),
        AnyElement::Col(col) => col.render(renderer, datas, events, uid),
        AnyElement::Button(button) => button.render(renderer, datas, events, uid),
        AnyElement::Row(row) => row.render(renderer, datas, events, uid),
        AnyElement::Center(center) => center.render(renderer, datas, events, uid),
        AnyElement::Checkbox(checkbox) => checkbox.render(renderer, datas, events, uid),
        AnyElement::Select(select) => select.render(renderer, datas, events, uid),
        AnyElement::Container(container) => container.render(renderer, datas, events, uid),
        AnyElement::FloatingElement(floating_element) => {
            floating_element.render(renderer, datas, events, uid)
        }
        AnyElement::Grid(grid) => grid.render(renderer, datas, events, uid),
        AnyElement::WindowSystem(window_system) => {
            window_system.render(renderer, datas, events, uid)
        }
        AnyElement::Radio(radio) => radio.render(renderer, datas, events, uid),
        AnyElement::Scroll(scroll) => scroll.render(renderer, datas, events, uid),
        AnyElement::Progress(progress) => progress.render(renderer, datas, events, uid),
        AnyElement::Trigger(trigger) => trigger.render(renderer, datas, events, uid),
    };
}

pub fn process_event_for_element<'a>(
    element: &'a mut AnyElement,
    event: XmlChangeEvent,
) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
    match element {
        AnyElement::Label(label) => label.process_event(&event),
        AnyElement::Col(col) => col.process_event(&event),
        AnyElement::Button(button) => button.process_event(&event),
        AnyElement::Row(row) => row.process_event(&event),
        AnyElement::Center(center) => center.process_event(&event),
        AnyElement::Checkbox(checkbox) => checkbox.process_event(&event),
        AnyElement::Select(select) => select.process_event(&event),
        AnyElement::Container(container) => container.process_event(&event),
        AnyElement::FloatingElement(floating_element) => floating_element.process_event(&event),
        AnyElement::Grid(grid) => grid.process_event(&event),
        AnyElement::WindowSystem(window_system) => window_system.process_event(&event),
        AnyElement::Radio(radio) => radio.process_event(&event),
        AnyElement::Scroll(scroll) => scroll.process_event(&event),
        AnyElement::Progress(progress) => progress.process_event(&event),
        AnyElement::Trigger(trigger) => trigger.process_event(&event),
    }
}
