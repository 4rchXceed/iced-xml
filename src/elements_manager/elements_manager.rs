use std::any::Any;
use std::collections::HashMap;

use iced::Subscription;
use iced::widget::text;

use crate::app_manager::utils::ComponentFunctions;
use crate::dom::events::{DomInternalMessageType, EventListenerTypes};
use crate::dom::query::{DomQuery, DomQueryType};
use crate::dom::query_builder::{EventResponse, QueryResponse};
use crate::rs_utils::get_unique_id;
use crate::xml_engine::Message;
use crate::xml_struct::elements::library::{
    process_event_callback_for_element, process_event_for_element, render_element,
};
use crate::xml_struct::theming::Fonts;
use crate::xml_struct::xml_element::XmlElement;

use super::elements_renderer::ElementsRenderer;
use super::mappings::ElementMappings;
use super::utils::{ElementEventResponse, EventListener, RenderChildParameters, RendererEvent};

pub struct ElementsManager {
    /// List of event listeners for the elements
    event_listeners: Vec<EventListener>,
    /// Stores the mappings of elements, their parents, and their virtual children
    mappings: ElementMappings,
    /// Stores the components as unknown types
    pub components: HashMap<i32, Box<dyn Any>>,
    /// Stores the functions to render and update components, since they are unknown types (dyn Any)
    pub functions: ComponentFunctions,
    /// Custom fonts, declared when the engine is initialized, and used when rendering elements
    /// Since FontFamily::Name() requires a &'static str
    fonts: Fonts,
    /// The renderer for the elements, which handles CSS and style changes
    renderer: ElementsRenderer,
}

impl ElementsManager {
    /// Creates a new ElementRenderer with the given fonts and component functions
    ///
    /// Parameters:
    /// - fonts: A Fonts struct (see Fonts)
    /// - functions: A ComponentFunctions struct (see ComponentFunctions)
    pub fn new(fonts: Fonts, functions: ComponentFunctions) -> Self {
        Self {
            mappings: ElementMappings::new(),
            event_listeners: Vec::new(),
            components: HashMap::new(),
            functions: functions,
            fonts: fonts.clone(),
            renderer: ElementsRenderer::new(fonts),
        }
    }

    /// Returns the data associated with a given element UID and key, if any
    /// Datas => data-* attributes
    ///
    /// Parameters:
    /// - element: The UID of the element to query
    /// - key: The key of the data to retrieve
    pub fn get_data(&mut self, element: i32, key: &str) -> Option<String> {
        let element = self.mappings.elements.get_mut(&element);
        if let Some((_, data)) = element {
            return data.xml_element.datas.get(key).cloned();
        } else {
            return None;
        }
    }

    // For-elements functions
    /// Returns a mutable reference to the list of event listeners
    pub fn get_event_listeners(&mut self) -> &mut Vec<EventListener> {
        return &mut self.event_listeners;
    }

    /// Returns a reference to the list of fonts
    pub fn get_font_list(&self) -> &Fonts {
        return &self.fonts;
    }

    /// Registers a component in the renderer, replacing any existing component with the same UID.
    ///
    /// Parameters:
    /// - uid: The unique identifier for the component.
    /// - component: The component to register, as a boxed trait object (Box<dyn
    pub fn register_component(&mut self, uid: i32, component: Box<dyn Any>) {
        self.mappings
            .remove_cascade(uid, false, &mut self.event_listeners);

        self.components.insert(uid, component);
    }

    /// Forwards the subscriptions of all registered components to the main application.
    pub fn subscribe_components(&self) -> Vec<Subscription<Message>> {
        return self
            .components
            .iter()
            .map(|(_, c)| (self.functions.subscribe)(c))
            .flatten()
            .collect();
    }

    /// Renders an element by its UID, applying any child parameters if provided.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the element to render.
    /// - child_data: Optional parameters to pass to child elements during rendering.
    ///
    /// Returns:
    /// - An iced::Element representing the rendered element.
    pub fn render_element<'a>(
        &self,
        uid: i32,
        child_data: Option<RenderChildParameters>,
    ) -> iced::Element<'_, Message> {
        if let Some(component) = self.components.get(&uid) {
            let render = self.functions.render;

            return render(component);
        }

        let element = self.mappings.elements.get(&uid);

        if let Some((element, datas)) = element {
            let events = self
                .event_listeners
                .iter()
                .filter(|v| v.target == uid)
                .collect::<Vec<&EventListener>>();

            // If the element is enabled
            if datas.default_theme.enable {
                let mut datas = datas.clone();

                datas.child_data = child_data;

                let output = render_element(element, self, datas, events, uid);

                return output;
            } else {
                return iced::widget::Space::new().height(0).width(0).into(); // Best thing I found so far to "hide" an element.
            }
        } else {
            return text(format!("Element with id {} not found", uid)).into();
        }
    }

    /// Passes an event to an element
    ///
    /// Parameters:
    /// - target_uid: The UID of the target element to pass the event to.
    /// - event_type: The type of the event to pass.
    /// - event_datas: The data associated with the event.
    ///
    /// Returns:
    /// - A QueryResponse indicating the result of the event processing (with additional data)
    pub fn forwards_event_to_element(
        &mut self,
        target_uid: i32,
        event_type: EventListenerTypes,
        event_datas: EventResponse,
    ) -> QueryResponse {
        if let Some((element, _)) = self.mappings.elements.get_mut(&target_uid) {
            let element_response =
                process_event_callback_for_element(element, &event_type, &event_datas);

            if let Some(element_response) = element_response {
                let event_response =
                    self.process_element_response(element_response, None, false, None);

                if let Some(event_response) = event_response {
                    return event_response;
                } else {
                    return QueryResponse::fail(
                        format!(
                            "Element with uid {} didn't return a response for event {:?}",
                            target_uid, event_type
                        )
                        .as_str(),
                    );
                }
            } else {
                return QueryResponse::fail(
                    format!(
                        "Element with uid {} doesn't have a callback for event {:?}",
                        target_uid, event_type
                    )
                    .as_str(),
                );
            }
        } else {
            return QueryResponse::fail(
                format!("Element with uid {} not found", target_uid).as_str(),
            );
        }
    }

    /// Emits an internal event to an element, handling style changes and forwarding events as necessary.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the target element.
    /// - event: The DomInternalMessageType representing the event to emit.
    /// - comes_from_hot_reload: A boolean indicating whether this event is coming from a hot-reload operation (use false by default).
    ///
    /// Returns:
    /// - A QueryResponse indicating the result of the event processing (with additional data)
    pub fn process_element_event(
        &mut self,
        uid: i32,
        event: DomInternalMessageType,
        comes_from_hot_reload: bool,
    ) -> QueryResponse {
        if let Some((element, _)) = self.mappings.elements.get_mut(&uid) {
            // If the event is a style change, we update the style for the element and don't forward the event to the element itself
            let element_response_op = match event {
                DomInternalMessageType::StyleChange(ref event) => {
                    self.renderer.update_style_for(
                        uid,
                        event.clone(),
                        comes_from_hot_reload,
                        &mut self.mappings,
                    );
                    None
                }
                _ => process_event_for_element(element, event.clone()),
            };

            // Post process the event response, if any, and forward it to the renderer or other elements
            if let Some(element_response) = element_response_op {
                let event_response = element_response.response.clone();

                // Post-process the event response
                let response = self.process_element_response(
                    element_response,
                    Some(event.clone()),
                    comes_from_hot_reload,
                    Some(event_response.clone()),
                );

                if let Some(response) = response {
                    return response;
                } else {
                    return event_response;
                }
            }
        }

        return QueryResponse::fail(
            format!(
                "Element with uid {} not found, or no response from element",
                uid
            )
            .as_str(),
        );
    }

    /// Processes the response from an element after an event has been emitted to it.
    /// This function handles forwarding events to other elements and emitting events to the renderer.
    ///
    /// Parameters:
    /// - element_response: The ElementEventResponse returned by the element after processing the event.
    /// - event: The DomInternalMessageType representing the original event that was emitted.
    /// - comes_from_hot_reload: A boolean indicating whether this event is coming from a hot-reload operation (use false by default).
    /// - event_response: An optional QueryResponse to concatenate with the new responses from forwarded events.
    ///
    /// Returns:
    /// - An optional QueryResponse that combines the original response with any new responses from forwarded events
    pub fn process_element_response(
        &mut self,
        element_response: ElementEventResponse,
        event_op: Option<DomInternalMessageType>,
        comes_from_hot_reload: bool,
        mut event_response: Option<QueryResponse>,
    ) -> Option<QueryResponse> {
        // Custom element events
        for renderer_event in element_response.renderer_events {
            match renderer_event {
                RendererEvent::RadioSelectionChange(id, value) => {
                    self.mappings.set_radio_selection(id, value);
                }
            }
        }

        // Forward the event to other elements, if any
        if let Some(event) = event_op {
            for target in element_response.forward_to {
                let new_response =
                    self.process_element_event(target, event.clone(), comes_from_hot_reload);

                if let Some(event_response) = &mut event_response {
                    event_response.concat(new_response);
                } else {
                    event_response = Some(new_response);
                }
            }
        }

        return event_response;
    }

    /// Registers an event listener for an element.
    /// Two cases possible:
    /// - If the event listener already exists for the element, we just add the new handler to the list of handlers for that event.
    /// - If the event listener doesn't exist for the element, we create a new EventListener and add it to the list of event listeners.
    ///
    /// Parameters:
    /// - event_type: The type of the event to listen for.
    /// - target: The UID of the target element to listen for events on.
    /// - handler: The unique identifier of the handler function to call when the event occurs.
    pub fn register_event(&mut self, event_type: EventListenerTypes, target: i32, handler: i32) {
        if let Some(event_listener) = self
            .event_listeners
            .iter_mut()
            .find(|e| e.event_type == event_type && e.target == target)
        {
            event_listener.handlers.push(handler);

            return;
        } else {
            self.event_listeners.push(EventListener {
                event_type: event_type,
                target: target,
                handlers: vec![handler],
                event_uid: get_unique_id(),
            });
        }
    }

    pub fn load_css(
        &mut self,
        css: &str,
        hot_reload: bool,
    ) -> Result<Vec<(i32, DomInternalMessageType, bool)>, String> {
        return self.renderer.load_css(css, hot_reload, &mut self.mappings);
    }

    pub fn remove_cascade(&mut self, uid: i32) -> Option<Box<XmlElement>> {
        return self
            .mappings
            .remove_cascade(uid, true, &mut self.event_listeners);
    }

    pub fn replace_element(&mut self, uid: i32, new_element: Box<XmlElement>) -> DomQuery {
        return self
            .mappings
            .replace_element(uid, new_element, &mut self.event_listeners);
    }

    pub fn raw_element_query(&self, query: &DomQueryType) -> Vec<i32> {
        return self.mappings.raw_element_query(query);
    }

    pub fn get_mappings(&self) -> &ElementMappings {
        return &self.mappings;
    }

    pub fn get_mappings_mut(&mut self) -> &mut ElementMappings {
        return &mut self.mappings;
    }

    pub fn event_listeners_mut(&mut self) -> &mut Vec<EventListener> {
        return &mut self.event_listeners;
    }
}
