// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub struct __EL__NAME {
    // If you have children, store them here
    // children: Vec<i32>,
}

impl ElementBase for __EL__NAME {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        // If it supports children, initialize them here with renderer.init_element
        // let mut children: Vec<i32> = Vec::new();
        // for child in &xml_element.children {
        //     children.push(renderer.init_element_from_xml(child));
        // }
        // If you need to create a text label, but still want theming, use this:
        // let virtual_text = renderer.init_element(
        //     AnyElement::Label(Label::virt(xml_element.text.clone())),
        //     None,
        //     Some(xml_element.theme.clone()),
        // );

        Self { children: children }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        theme: &'a XmlTheme,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        // Declare your element
        // let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();

        // Set up your styles
        // container = container
        //     .clip(theme.clip)
        //     .height(theme.height)
        //     .padding(theme.padding)
        //     .width(theme.width)
        //     .max_width(theme.max_width)
        //     .spacing(theme.spacing)
        //     .align_x(theme.align_x);

        // Register any events here
        // for event in events {
        //     match event.event_type.as_str() {
        //         "click" => {
        //             button = button.on_press(Message::DomEvent(
        //                 event.event_uid,
        //                 EventResponse::new(self_uid),
        //             ));
        //         }
        //         _ => (),
        //     }
        // }

        return container.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<(QueryResponse, Vec<i32>)> {
        match event {
            // Process PropertyChange / GetProperty / Custom events
            // The second parameter is a list of element IDs to forward the event to
            // For example, if you want to forward the event to your "virtual label element", so it changes text,
            // you would use `Some(..., vec![virtual_text])`
            _ => None,
        }
    }
}
