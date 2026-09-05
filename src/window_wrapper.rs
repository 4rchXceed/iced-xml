//! This module contains the base trait that every window must implement. It also contains some utils (structs and enums
//! TODO: doc
use std::any::Any;

use crate::{
    app_manager::App,
    dom::{
        events::DomQueryBuilder,
        query_builder::{QueryBuilder, QueryResponse},
    },
    xml_engine::{Message, XmlEngine},
    xml_struct::{
        elements::library::{ElementError, generate_element_from_tag},
        parser::XmlElement,
    },
};

// Type alias for the result type used in the application
pub type AppResult = iced::Result;

// Type aliases for CSS watcher functionality, conditionally compiled based on the "css-watcher" feature
#[cfg(feature = "css-watcher")]
pub type CssChannelType = notify::Result<notify::Event>;
#[cfg(feature = "css-watcher")]
pub type CssRx = std::sync::mpsc::Receiver<CssChannelType>;
#[cfg(feature = "css-watcher")]
pub type CssTx = std::sync::mpsc::Sender<CssChannelType>;
#[cfg(feature = "css-watcher")]
pub type CssWatcher = notify::RecommendedWatcher;

///  Struct to hold references to the XmlEngine and QueryBuilder, along with optional CSS watcher components
///
///  This is used to pass around the necessary objects for window templates
pub struct Objects<'a, WindowApp, AppState> {
    pub engine: &'a mut XmlEngine,
    pub qb: &'a mut QueryBuilder<WindowApp, AppState>,
    #[cfg(feature = "css-watcher")]
    pub css_watcher_rx: Option<&'a mut CssRx>,
    #[cfg(feature = "css-watcher")]
    pub css_path: &'a str,
}

///  Struct to hold references to the XmlEngine and QueryBuilder for read-only access
pub struct ObjectsReadOnly<'a, WindowApp, AppState> {
    pub engine: &'a XmlEngine,
    pub qb: &'a QueryBuilder<WindowApp, AppState>,
}

///  Every remove_component() errors
pub enum ComponentRemoveError {
    // Component with uid (i32) isn't found / has already been supressed
    ComponentUidNotFound(i32),
    // Tag <Void /> not found
    ElementError(ElementError), // Shouldn't happen
}

///  Trait for defining the interface of a window template
///
///  This trait provides methods for interacting with the XmlEngine, QueryBuilder, and handling events such as updates, rendering, and subscriptions.
///
///  You (user of this lib) need to fill:
///
///  - get_objects
///
///  - get_objects_read_only
///
///  - get_self
///
///  - on_close
///
///  - into_any
///
///  - set_css_watcher_rx (if using CSS watcher)
pub trait WindowTemplate<WindowApp: 'static, AppState: 'static> {
    ///  Get the mutable references to the XmlEngine and QueryBuilder, along with optional CSS watcher components
    fn get_objects(&mut self) -> Objects<'_, WindowApp, AppState>;
    ///  Get the read-only references to the XmlEngine and QueryBuilder
    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, WindowApp, AppState>;
    ///  Get a mutable reference to the window application itself
    fn get_self(&mut self) -> &mut WindowApp;

    ///  Set the CSS watcher receiver, if using the "css-watcher" feature
    ///
    /// Parameters:
    ///
    /// - rx: the receiver for CSS watcher events
    ///
    /// Returns: nothing
    #[cfg(feature = "css-watcher")]
    fn set_css_watcher_rx(&mut self, rx: CssRx);

    ///  When the window is closed
    ///
    /// Parameters:
    ///
    /// - app_state: the application state
    ///
    /// Returns: nothing
    #[allow(unused_variables)]
    fn on_close(&mut self, app_state: &mut App<AppState>) {}

    ///  Util to convert the window template into a Box<dyn Any>
    ///
    ///  Here: just paste `return self;`
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    ///  Update the window template with a new message and application state
    fn update(&mut self, message: crate::xml_engine::Message, app_state: &mut App<AppState>) {
        // Get the engine
        let me = self.get_objects();
        // Update all components with the new message
        for (_, component) in me.engine.element_renderer.components.iter_mut() {
            (me.engine.element_renderer.functions.update)(component, message.clone(), app_state);
        }
        // Update the engine and get the responses
        let responses = me.engine.update(message);
        for (callback, response) in me.qb.fetch(responses) {
            let me = self.get_self();
            // Call the callback with the window app, response, and application state
            callback(me, response, app_state);
        }
    }

    ///  Add a component to the window template, given a DOM query result (Dom::...) and a boxed component
    fn add_component(
        &mut self,
        element: DomQueryBuilder,
        component: Box<dyn Any>,
    ) -> Result<i32, &str> {
        // Get the engine and query builder
        let me = self.get_objects();
        let uid = me
            .engine
            .element_renderer
            .raw_element_query(&element.get_query().query_type);
        // Ensure that only one element was found for the query
        if uid.len() != 1 {
            return Err(
                "Panic: More than one element found when adding a component. Please ensure that the query is unique.",
            );
        }
        // Register the component with the engine's element renderer
        me.engine
            .element_renderer
            .register_component(uid[0], component);
        return Ok(uid[0]);
    }

    ///  Remove a component from the window template, given its unique identifier and the application state
    ///
    ///  component_uid: the UID that was returned by add_component(...)
    ///
    ///  app_state, the app state
    ///
    ///  Returns a Result<i32, ComponentRemoveError>
    ///
    ///  i32 -> the component element id
    fn remove_component(
        &mut self,
        component_uid: i32,
        app_state: &mut App<AppState>,
    ) -> Result<i32, ComponentRemoveError> {
        // Get the engine and query builder
        let me = self.get_objects();
        let elem_renderer = &mut me.engine.element_renderer;
        // Remove the component
        let component = elem_renderer.components.remove(&component_uid);
        if component.is_none() {
            return Err(ComponentRemoveError::ComponentUidNotFound(component_uid));
        }
        // Call the on_close callback when closing
        (elem_renderer.functions.on_close)(&mut component.unwrap(), app_state);
        // The create an empty element replacing the old component, so it's parent doesn't crash
        let element = generate_element_from_tag(&XmlElement::void(), elem_renderer, component_uid);
        // If the element has been created
        if element.is_ok() {
            let element_uid = element.unwrap();
            // Add the element to the DOM
            elem_renderer.init_element(element_uid, Some(XmlElement::void()), None, component_uid);
            return Ok(component_uid);
        } else {
            return Err(ComponentRemoveError::ElementError(element.err().unwrap()));
        }
    }
    ///  This is processes all the query builder queries and execute them
    ///
    ///  You need to call this after EVERY block of query builder queries
    ///
    ///  Returns a QueryResponse, which contains any potential errors about the queries. See comments for QueryResponse.
    fn process(&mut self) -> QueryResponse {
        // Returns the objects
        let me = self.get_objects();
        // Execute all queries
        for query in me.qb.execute(me.engine) {
            let me = self.get_self();
            // Run the callbacks
            (query.0)(me, query.1);
        }
        let me = self.get_objects();
        // Returns the last query, so we can use this, without using then and loosing access to local variables
        return me.qb.last.clone();
    }
    // use the "render" helper
    // Basically renders the elements into an Iced Element
    fn render(&self) -> iced::Element<'_, crate::xml_engine::Message> {
        let me = self.get_objects_read_only();
        return me.engine.view();
    }
    // Subscription logic (for set_timeout and set_interval)
    // Subscribes to any dynamic events.
    // A dynamic event is a future event that will not created by the user
    fn subscription(&self) -> Vec<iced::Subscription<Message>> {
        // Get the engine
        let engine = self.get_objects_read_only().engine;
        // Get the builder
        let query_builder = self.get_objects_read_only().qb;
        // Run the query builder's subscribe function
        return query_builder.subscribe(engine);
    }
}
