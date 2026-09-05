use crate::{
    dom::{events::DomInternalMessageType, query_builder::EventResponse},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::{
            button::Button, center::Center, checkbox::Checkbox, col::Col, container::Container,
            element_base::ElementBase, float::FloatingElement, grid::Grid, input::Input,
            label::Label, progress::Progress, radio::RadioButton, range::Range, row::Row,
            scrollable::Scroll, select::Select, space::Space, table::Table, textarea::Textarea,
            toggle::Toggle, tooltip::Tooltip, trigger::Trigger, var::Var, void::Void,
            window_system::WindowSystem,
        },
        parser::XmlElement,
    },
};

pub enum AnyElement {
    Button(Button),
    Center(Center),
    Checkbox(Checkbox),
    Col(Col),
    Container(Container),
    FloatingElement(FloatingElement),
    Grid(Grid),
    Input(Input),
    Label(Label),
    Progress(Progress),
    Radio(RadioButton),
    Row(Row),
    Range(Range),
    Scroll(Scroll),
    Select(Select),
    WindowSystem(WindowSystem),
    Table(Table),
    Textarea(Textarea),
    Toggle(Toggle),
    Tooltip(Tooltip),
    Trigger(Trigger),
    Var(Var),
    Space(Space),
    Void(Void),
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
        "Space" => Some(AnyElement::Space(Space::new(xml_element, renderer, self_uid))),
        "Trigger" => Some(AnyElement::Trigger(Trigger::new(xml_element, renderer, self_uid))),
        "Table" => Some(AnyElement::Table(Table::new(xml_element, renderer, self_uid))),
        "Var" => Some(AnyElement::Var(Var::new(xml_element, renderer, self_uid))),
        "Input" => Some(AnyElement::Input(Input::new(xml_element, renderer, self_uid))),
        "Textarea" => Some(AnyElement::Textarea(Textarea::new(xml_element, renderer, self_uid))),
        "Toggle" => Some(AnyElement::Toggle(Toggle::new(xml_element, renderer, self_uid))),
        "Tooltip" => Some(AnyElement::Tooltip(Tooltip::new(xml_element, renderer, self_uid))),
        "Range" => Some(AnyElement::Range(Range::new(xml_element, renderer, self_uid))),
        "Void" => Some(AnyElement::Void(Void::new(xml_element, renderer, self_uid))),
        _ => None,
    };
}

pub fn render_element<'a>(
    element: &'a AnyElement,
    renderer: &'a ElementRenderer,
    datas: ElementExtraData,
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
        AnyElement::Space(space) => space.render(renderer, datas, events, uid),
        AnyElement::Trigger(trigger) => trigger.render(renderer, datas, events, uid),
        AnyElement::Table(table) => table.render(renderer, datas, events, uid),
        AnyElement::Var(var) => var.render(renderer, datas, events, uid),
        AnyElement::Input(input) => input.render(renderer, datas, events, uid),
        AnyElement::Textarea(textarea) => textarea.render(renderer, datas, events, uid),
        AnyElement::Toggle(toggle) => toggle.render(renderer, datas, events, uid),
        AnyElement::Tooltip(tooltip) => tooltip.render(renderer, datas, events, uid),
        AnyElement::Range(range) => range.render(renderer, datas, events, uid),
        AnyElement::Void(void) => void.render(renderer, datas, events, uid),
    };
}

pub fn process_event_for_element<'a>(
    element: &'a mut AnyElement,
    event: DomInternalMessageType,
) -> Option<ElementEventResponse> {
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
        AnyElement::Space(space) => space.process_event(&event),
        AnyElement::Trigger(trigger) => trigger.process_event(&event),
        AnyElement::Table(table) => table.process_event(&event),
        AnyElement::Var(var) => var.process_event(&event),
        AnyElement::Input(input) => input.process_event(&event),
        AnyElement::Textarea(textarea) => textarea.process_event(&event),
        AnyElement::Toggle(toggle) => toggle.process_event(&event),
        AnyElement::Tooltip(tooltip) => tooltip.process_event(&event),
        AnyElement::Range(range) => range.process_event(&event),
        AnyElement::Void(void) => void.process_event(&event),
    }
}

pub fn process_event_callback_for_element<'a>(
    element: &'a mut AnyElement,
    event_name: &String,
    event_response: &EventResponse,
) -> Option<ElementEventResponse> {
    match element {
        AnyElement::Label(label) => label.event_callback(event_name, event_response),
        AnyElement::Col(col) => col.event_callback(event_name, event_response),
        AnyElement::Button(button) => button.event_callback(event_name, event_response),
        AnyElement::Row(row) => row.event_callback(event_name, event_response),
        AnyElement::Center(center) => center.event_callback(event_name, event_response),
        AnyElement::Checkbox(checkbox) => checkbox.event_callback(event_name, event_response),
        AnyElement::Select(select) => select.event_callback(event_name, event_response),
        AnyElement::Container(container) => container.event_callback(event_name, event_response),
        AnyElement::FloatingElement(floating_element) => {
            floating_element.event_callback(event_name, event_response)
        }
        AnyElement::Grid(grid) => grid.event_callback(event_name, event_response),
        AnyElement::WindowSystem(window_system) => {
            window_system.event_callback(event_name, event_response)
        }
        AnyElement::Radio(radio) => radio.event_callback(event_name, event_response),
        AnyElement::Scroll(scroll) => scroll.event_callback(event_name, event_response),
        AnyElement::Progress(progress) => progress.event_callback(event_name, event_response),
        AnyElement::Space(space) => space.event_callback(event_name, event_response),
        AnyElement::Trigger(trigger) => trigger.event_callback(event_name, event_response),
        AnyElement::Table(table) => table.event_callback(event_name, event_response),
        AnyElement::Var(var) => var.event_callback(event_name, event_response),
        AnyElement::Input(input) => input.event_callback(event_name, event_response),
        AnyElement::Textarea(textarea) => textarea.event_callback(event_name, event_response),
        AnyElement::Toggle(toggle) => toggle.event_callback(event_name, event_response),
        AnyElement::Tooltip(tooltip) => tooltip.event_callback(event_name, event_response),
        AnyElement::Range(range) => range.event_callback(event_name, event_response),
        AnyElement::Void(void) => void.event_callback(event_name, event_response),
    }
}
