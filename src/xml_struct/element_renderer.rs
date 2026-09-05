use std::{any::Any, collections::HashMap};

use iced::{Subscription, widget::text};

use crate::{
    app_manager::ComponentFunctions,
    css_reader::{CssReader, Rule, RuleBlock, Selector},
    dom::{
        events::{DomInternalMessageType, EventListenerTypes},
        query::{ComplexQuery, ComplexQueryJoinType, DomQuery, DomQueryType},
        query_builder::{EventResponse, QueryResponse},
    },
    rs_utils::get_unique_id,
    xml_engine::Message,
    xml_struct::{
        elements::{
            library::{
                AnyElement, generate_element_from_tag, process_event_callback_for_element,
                process_event_for_element, render_element,
            },
            radio::RadioElement,
        },
        hot_reload::cleanup_for_hot_reload,
        parser::XmlElement,
        theming::{Fonts, XmlTheme, gen_styles},
    },
};

/// Datas that can be passed as "parameters" to children elements
#[derive(Clone, Debug)]
pub struct RenderChildParameters {
    pub table_datas: Option<HashMap<String, String>>, // For table elements
}

impl Default for RenderChildParameters {
    fn default() -> Self {
        Self { table_datas: None }
    }
}

/// Represents a style change event (key-value pair)
/// This will be used to update the style of an element when a CSS rule is applied or changed.
#[derive(Debug, Clone, Hash)]
pub struct StyleChangeEvent {
    pub key: String,
    pub value: String,
    pub custom_flag: Option<String>,
}

/// This represents an event listener that can be attached to an element.
pub struct EventListener {
    /// See EventListenerTypes for the list of events that can be listened to
    pub event_type: EventListenerTypes,
    /// Target element UID
    pub target: i32,
    /// The list of callbacks to call when the event is triggered
    pub handlers: Vec<i32>, // List of callbacks to forward the event to
    /// Unique ID for the event
    pub event_uid: i32,
}

/// Event to fire from an element to the renderer
pub enum RendererEvent {
    // If elements wants to emit an event to the renderer, it can use this enum
    RadioSelectionChange(String, RadioElement), // (selection_id, selected_element)
}

/// This struct represents a state (for one element) at an instant T, to be able to revert style changes when hot-reloading CSS.
#[derive(Clone, Debug)]
pub(super) struct HotReloadStateEntry {
    pub default_theme: XmlTheme,
    pub flag_themes: HashMap<String, XmlTheme>,
}

/// This struct represents the state of the renderer (style) at an instant T, to be able to revert style changes when hot-reloading CSS.
#[derive(Clone)]
pub(super) struct HotReloadState {
    pub rules: Vec<RuleBlock>,
    pub state: HashMap<i32, HotReloadStateEntry>,
}

/// This struct represents the extra data associated with an element, such as its style, XML Source, ...
#[derive(Clone, Debug)]
pub struct ElementExtraData {
    /// The Main theme
    pub default_theme: XmlTheme,
    /// Themes for "sub-elements" (::for(XYZ))
    pub flag_themes: HashMap<String, XmlTheme>,
    /// It's XML "Source"
    pub xml_element: XmlElement,
    /// Parameters from parents
    pub child_data: Option<RenderChildParameters>,
}

/// This struct represents the response of an element to an event.
/// It contains the response to the event, the list of elements to forward the event to, and the list of events to forward to the renderer.
pub struct ElementEventResponse {
    /// Direct response to the event, can be success or fail, and carry additional data (like a string, ...)
    pub response: QueryResponse,
    /// List of elements to forward the event to
    pub forward_to: Vec<i32>, // List of elements to forward the event to
    /// List of events to forward to the renderer
    pub renderer_events: Vec<RendererEvent>, // List of events to forward to the renderer
}

impl ElementEventResponse {
    /// Creates a new ElementEventResponse with the given response
    ///
    /// Parameters:
    /// - response: the QueryResponse, useful to pass information about the event (additional data)
    pub fn new(response: QueryResponse) -> Self {
        Self {
            response: response,
            forward_to: Vec::new(),
            renderer_events: Vec::new(),
        }
    }

    /// Creates a successful event response
    pub fn success() -> Self {
        Self {
            response: QueryResponse::success(),
            forward_to: Vec::new(),
            renderer_events: Vec::new(),
        }
    }

    /// Adds elements to forward the event to
    ///
    /// Parameters:
    /// - forward_to: the list of elements to forward the event to (by UID)
    pub fn with_forward_to(mut self, forward_to: Vec<i32>) -> Self {
        self.forward_to = forward_to;
        self
    }

    /// Adds events to forward to the renderer
    ///
    /// Parameters:
    /// - renderer_events: the list of events to forward to the renderer (see RendererEvent)
    pub fn with_renderer_events(mut self, renderer_events: Vec<RendererEvent>) -> Self {
        self.renderer_events = renderer_events;
        self
    }
}

/// "Hack" to store the render and update functions for components, since they are unknown types (dyn Any)
/// Will need to figure out how to make this better
pub type RenderComponentFn = fn(&Box<dyn Any>) -> iced::Element<'_, Message>;
/// "Hack" to store the render and update functions for components, since they are unknown types (dyn Any)
/// Will need to figure out how to make this better
pub type UpdateComponentFn = fn(&Box<dyn Any>, Message) -> iced::Task<Message>;

/// This is the "core" of the engine, it stores all the elements, their styles, their events, and their relationships.
/// TODO: Doc
pub struct ElementRenderer {
    /// List of event listeners for the elements
    event_listeners: Vec<EventListener>,
    /// The elements stored in the renderer, with their extra data (see ElementExtraData)
    pub(super) elements: HashMap<i32, (AnyElement, ElementExtraData)>,
    /// Maps the element's ID to its UID, for quick access
    id_map: HashMap<String, i32>,
    /// Maps the element's class to its UIDs, for quick access
    classes_map: HashMap<String, Vec<i32>>,
    /// Maps the element's tag to its UIDs, for quick access
    tags_map: HashMap<String, Vec<i32>>,
    /// Maps the element's UID to its parent's UID, for quick access
    parent_map: HashMap<i32, i32>, // key: child, value: parent
    /// Maps the element's UID to its XML source, for quick access
    sources_map: HashMap<i32, XmlElement>, // key: source name, value: XmlElement
    /// Maps the element's UID to its virtual children UIDs, for quick access
    /// Virtual childrens are elements that are not declared in the XML, but are created by an element.
    /// I'm using this system, so the sub-elements are still stylable by CSS
    /// TODO: Add a better system, or remove it entirely. I'll probably use the flag system
    virtual_elements: HashMap<i32, Vec<i32>>, // key: parent_uid, value: virtual children uids
    /// Stores the state of the renderer for hot-reloading CSS, if any
    pub(super) hot_reload_states: Option<HotReloadState>,
    /// Stores the components as unknown types, to be downcasted later when needed, else I need to type (<T>) the whole elementrenderer, with all sub-elements, queries etc.
    /// I know it's not the best solution, but it's way better than having everything typed
    pub components: HashMap<i32, Box<dyn Any>>,
    /// Stores the functions to render and update components, since they are unknown types (dyn Any)
    pub functions: ComponentFunctions,
    // Custom storage for elements
    /// Stores the selected radio button for each radio group, by ID
    radio_button_map: HashMap<String, RadioElement>,
    /// This one's tricky: if an element needs to store a string, but it needs to have the Copy trait, it can use this
    string_map: HashMap<i32, String>,
    /// Custom fonts, declared when the engine is initialized, and used when rendering elements
    /// Since FontFamily::Name() requires a &'static str
    fonts: Fonts,
}

impl ElementRenderer {
    /// Creates a new ElementRenderer with the given fonts and component functions
    ///
    /// Parameters:
    /// - fonts: A Fonts struct (see Fonts)
    /// - functions: A ComponentFunctions struct (see ComponentFunctions)
    pub fn new(fonts: Fonts, functions: ComponentFunctions) -> Self {
        Self {
            elements: HashMap::new(),
            id_map: HashMap::new(),
            classes_map: HashMap::new(),
            tags_map: HashMap::new(),
            parent_map: HashMap::new(),
            event_listeners: Vec::new(),
            hot_reload_states: None,
            virtual_elements: HashMap::new(),
            sources_map: HashMap::new(),
            components: HashMap::new(),
            functions: functions,
            // Specific elements for radiobuttons comm
            radio_button_map: HashMap::new(),
            string_map: HashMap::new(),
            fonts: fonts,
        }
    }

    /// Returns a mutable reference to the list of event listeners
    pub fn get_event_listeners(&mut self) -> &mut Vec<EventListener> {
        return &mut self.event_listeners;
    }

    /// Returns a reference to the list of fonts
    pub fn get_font_list(&self) -> &Fonts {
        return &self.fonts;
    }

    /// Sets the selected radio button for a given radio group ID
    pub fn set_radio_selection(&mut self, id: String, value: RadioElement) {
        self.radio_button_map.insert(id, value);
    }

    /// Returns the selected radio button for a given radio group ID, if any
    pub fn get_radio_selection(&self, id: String) -> Option<RadioElement> {
        return self.radio_button_map.get(&id).cloned();
    }

    /// Registers a string in the string map and returns its unique ID
    pub fn register_stringdb(&mut self, str: String) -> i32 {
        let id = get_unique_id();
        self.string_map.insert(id, str);
        return id;
    }

    /// Returns the string associated with a given unique ID, if any
    pub fn get_stringdb(&self, id: i32) -> Option<String> {
        return self.string_map.get(&id).cloned();
    }

    /// Loads a CSS string into the renderer.
    /// Uses CssReader to parse the CSS and apply the rules to the elements.
    /// Handles hot-reloading by updating the state and reverting old styles if necessary.
    ///
    /// Parameters:
    /// - css: The CSS string to load.
    /// - hot_reload: A boolean indicating whether this is a hot-reload operation (use false by default)
    pub fn load_css(&mut self, css: &str, hot_reload: bool) -> (bool, String) {
        let mut reader = CssReader::new(css);
        reader.parse();
        if reader.get_kill_switch() {
            return (false, reader.get_kill_message());
        }
        if hot_reload {
            if self.hot_reload_states.is_some() {
                self.update_state(&reader.get_rules());
                cleanup_for_hot_reload(self, &reader.get_rules());
            } else {
                self.hot_reload_states = Some(self.generate_state(&reader.get_rules()));
            }
        }
        for rule_block in reader.get_rules() {
            let selectors = &rule_block.selectors;
            for selector in selectors {
                self.apply_rules(selector, &rule_block.rules, hot_reload);
            }
        }
        return (true, String::new());
    }

    /// Returns the data associated with a given element UID and key, if any
    /// Datas => data-* attributes
    ///
    /// Parameters:
    /// - element: The UID of the element to query
    /// - key: The key of the data to retrieve
    pub fn get_data(&mut self, element: i32, key: &str) -> Option<String> {
        let element = self.elements.get_mut(&element);
        if element.is_some() {
            let (_, data) = element.unwrap();
            return data.xml_element.datas.get(key).cloned();
        } else {
            return None;
        }
    }

    /// Generates the current state of the renderer for hot-reloading CSS.
    /// This function creates a snapshot of the current styles of all elements, which can be used to revert styles when hot-reloading CSS.
    fn generate_state(&mut self, rules: &Vec<RuleBlock>) -> HotReloadState {
        let mut state: HashMap<i32, HotReloadStateEntry> = HashMap::new();
        for (uid, (_, datas)) in &self.elements {
            state.insert(
                *uid,
                HotReloadStateEntry {
                    default_theme: datas.default_theme.clone(),
                    flag_themes: datas.flag_themes.clone(),
                },
            );
        }
        return HotReloadState {
            rules: rules.clone(),
            state: state,
        };
    }

    /// Updates the current state of the renderer for hot-reloading CSS.
    ///
    /// Parameters:
    /// - rules: the new rules to update the state with
    fn update_state(&mut self, rules: &Vec<RuleBlock>) {
        // Update only new rules, and keep the old state for the rest
        let mut state_hashed: HashMap<String, (Selector, Rule)> = HashMap::new();
        for rule_block in self.hot_reload_states.as_ref().unwrap().rules.iter() {
            for selector in &rule_block.selectors {
                for rule in &rule_block.rules {
                    state_hashed.insert(
                        rule.hash_with_selector(selector),
                        (selector.clone(), rule.clone()),
                    );
                }
            }
        }

        for rule_block in rules.iter() {
            for selector in &rule_block.selectors {
                for rule in &rule_block.rules {
                    let hash = rule.hash_with_selector(selector);
                    if !state_hashed.contains_key(&hash) {
                        // Push the new rule
                        self.hot_reload_states
                            .as_mut()
                            .unwrap()
                            .rules
                            .push(rule_block.clone());
                    }
                }
            }
        }
    }

    /// Applies the given CSS rules to the elements selected by the given selector.
    ///
    /// Parameters:
    /// - selector: The element selector to apply the rules to.
    /// - rules: The CSS rules to apply.
    /// - comes_from_hot_reload: A boolean indicating whether this is a hot-reload operation (use false by default)
    pub fn apply_rules(
        &mut self,
        selector: &Selector,
        rules: &Vec<Rule>,
        comes_from_hot_reload: bool,
    ) {
        let dom_query = DomQuery::new(
            selector.selector_type.clone(),
            selector.content.clone(),
            selector.flag.clone(),
        );

        let elements = self.element_query(&dom_query);
        let mut custom_style_for_flag = None;
        if selector.flag.is_some() {
            custom_style_for_flag = extract_selector_style_flag(&selector.flag.as_ref().unwrap());
        }
        for element in elements {
            for rule in rules.iter() {
                self.emit_internal_event(
                    element,
                    DomInternalMessageType::StyleChange(StyleChangeEvent {
                        key: rule.name.clone(),
                        value: rule.value.clone(),
                        custom_flag: custom_style_for_flag.clone(),
                    }),
                    comes_from_hot_reload,
                );
            }
        }
    }

    /// Post-processes the result of a query based on the query's flag.
    /// This is used to handle special cases like virtual elements or style flags.
    ///
    /// Parameters:
    /// - query: The DomQuery that was executed.
    /// - element: The UID of the element that was found by the query.
    ///
    /// Returns:
    /// - A vector of elements UID
    pub(crate) fn post_process_query_result(&self, query: &DomQuery, element: i32) -> Vec<i32> {
        if query.flag.is_some() {
            let flag = query.flag.as_ref().unwrap();
            return match flag.as_str() {
                "virtuals" => {
                    let virtual_children = self.virtual_elements.get(&element);
                    if virtual_children.is_some() {
                        virtual_children.unwrap().clone()
                    } else {
                        vec![]
                    }
                }
                _ => {
                    if extract_selector_style_flag(flag).is_some() {
                        vec![element]
                    } else {
                        vec![]
                    }
                }
            };
        } else {
            return vec![element];
        }
    }

    /// Executes a raw query on the elements without any post-processing.
    ///
    /// Parameters:
    /// - query: The DomQueryType to execute.
    ///
    /// Returns:
    /// - A vector of elements UID that match the query.
    pub fn raw_element_query(&self, query: &DomQueryType) -> Vec<i32> {
        return match &query {
            DomQueryType::ById(id) => {
                if let Some(uid) = self.id_map.get(id) {
                    vec![*uid]
                } else {
                    vec![]
                }
            }
            DomQueryType::ByUid(uid) => {
                if self.elements.contains_key(&uid) {
                    vec![uid.clone()]
                } else {
                    vec![]
                }
            } // _ => None,
            DomQueryType::Class(class) => {
                if let Some(uids) = self.classes_map.get(class) {
                    uids.clone()
                } else {
                    vec![]
                }
            }
            DomQueryType::Tag(tag) => {
                if let Some(uids) = self.tags_map.get(tag) {
                    uids.clone()
                } else {
                    vec![]
                }
            }
            DomQueryType::All => self.elements.keys().cloned().collect(),
            DomQueryType::Unused => vec![],
            DomQueryType::Complex(raw_complex_query) => {
                return self.run_complex_query(raw_complex_query.clone());
            }
        };
    }

    /// Executes a query on the elements and applies post-processing based on the query's flag.
    /// This is the main function to use when querying elements.
    ///
    /// Parameters:
    /// - query: The DomQuery to execute.
    ///
    /// Returns:
    /// - A vector of elements UID that match the query and post-processing.
    pub fn element_query(&self, query: &DomQuery) -> Vec<i32> {
        let query_result = self.raw_element_query(&query.query_type);
        return query_result
            .iter()
            .map(|e| self.post_process_query_result(query, e.clone()))
            .flatten()
            .collect();
    }

    /// Initializes an element from its XML representation and associates it with a parent element.
    ///
    /// Parameters:
    /// - xml_element: The XmlElement to initialize.
    /// - parent_uid: The UID of the parent element.
    ///
    /// Returns:
    /// - The UID of the newly created element.
    pub fn init_element_from_xml(&mut self, xml_element: &XmlElement, parent_uid: i32) -> i32 {
        let id = get_unique_id();
        self.parent_map.insert(id, parent_uid);
        return self.create_element_safe(xml_element.clone(), id, None);
    }

    /// Initializes a virtual element (not declared in the XML) and associates it with a parent element.
    /// This is useful for elements that generate sub-elements dynamically, like a button with a label.
    ///
    /// Parameters:
    /// - element: The AnyElement to initialize.
    /// - parent_theme: The XmlTheme of the parent element, if any.
    /// - parent_uid: The UID of the parent element.
    pub fn init_element_virt(
        &mut self,
        element: AnyElement,
        parent_theme: Option<XmlTheme>,
        parent_uid: i32,
    ) -> i32 {
        let uid = get_unique_id(); // "Backup"
        self.init_element(element, None, parent_theme, uid);
        let parent_virtual_children = self.virtual_elements.get_mut(&parent_uid);
        if parent_virtual_children.is_some() {
            self.virtual_elements
                .get_mut(&parent_uid)
                .unwrap()
                .push(uid);
        } else {
            self.virtual_elements.insert(parent_uid, vec![uid]);
        }
        return uid;
    }

    /// "Post-init" an element, after it has been created, to set its XML representation and theme.
    pub fn init_element(
        &mut self,
        element: AnyElement,
        xml: Option<XmlElement>,
        parent_theme: Option<XmlTheme>,
        uid: i32,
    ) {
        let mut xml_element;
        if let Some(xml) = xml {
            xml_element = xml;
        } else {
            xml_element = XmlElement::virt();
            if parent_theme.is_some() {
                xml_element.theme = parent_theme.unwrap();
            }
        }

        if xml_element.id.is_some() {
            self.id_map
                .insert(xml_element.id.clone().unwrap().to_string(), uid);
        }
        for class in &xml_element.classes {
            let class_map = self.classes_map.get(class);
            if class_map.is_some() {
                self.classes_map.get_mut(class).unwrap().push(uid);
            } else {
                self.classes_map.insert(class.clone(), vec![uid]);
            }
        }
        let tag_map = self.tags_map.get(&xml_element.tag);
        if tag_map.is_some() {
            self.tags_map.get_mut(&xml_element.tag).unwrap().push(uid);
        } else {
            self.tags_map.insert(xml_element.tag.clone(), vec![uid]);
        }
        self.sources_map.insert(uid, xml_element.clone());
        self.elements.insert(
            uid,
            (
                element,
                ElementExtraData {
                    default_theme: xml_element.theme.clone(),
                    flag_themes: HashMap::new(),
                    xml_element: xml_element,
                    child_data: None,
                },
            ),
        );
    }

    /// Registers a component in the renderer, replacing any existing component with the same UID.
    ///
    /// Parameters:
    /// - uid: The unique identifier for the component.
    /// - component: The component to register, as a boxed trait object (Box<dyn
    pub fn register_component(&mut self, uid: i32, component: Box<dyn Any>) {
        self.remove_cascade(uid, false);
        self.components.insert(uid, component);
    }

    /// Forwards the subscriptions of all registered components to the main application.
    pub fn subscribe_components(&self) -> Vec<Subscription<Message>> {
        let mut subscriptions = Vec::new();
        for (_, component) in self.components.iter() {
            subscriptions.append(&mut (self.functions.subscribe)(component));
        }
        return subscriptions;
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
        if self.components.contains_key(&uid) {
            let component = self.components.get(&uid).unwrap();
            return (self.functions.render)(component);
        }
        let element = self.elements.get(&uid);
        if element.is_some() {
            let events = self
                .event_listeners
                .iter()
                .filter(|v| v.target == uid)
                .collect::<Vec<&EventListener>>();
            let (element, datas) = element.unwrap();
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

    /// Registers a style change event for hot-reloading, updating the stored state of the element's styles.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the element where the style change occurred.
    /// - event: The StyleChangeEvent containing the key, value, and optional custom flag for the style change.
    pub fn register_hot_reload_change(&mut self, uid: i32, event: &StyleChangeEvent) {
        let hot_reload_state = self.hot_reload_states.as_mut().unwrap();
        let old_theme_op = hot_reload_state.state.get(&uid);
        if old_theme_op.is_some() {
            let mut old_theme = old_theme_op.unwrap().clone();
            if event.custom_flag.is_some() {
                gen_styles(
                    &event.key,
                    &event.value,
                    &mut old_theme
                        .flag_themes
                        .entry(event.custom_flag.clone().unwrap())
                        .or_insert(old_theme.default_theme.clone()),
                    &self.fonts,
                );
            } else {
                gen_styles(
                    &event.key,
                    &event.value,
                    &mut old_theme.default_theme,
                    &self.fonts,
                );
            }
        }
    }

    /// Updates the style for a specific element.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the element to update.
    /// - event: The StyleChangeEvent containing the key, value, and optional custom flag for the style change.
    /// - comes_from_hot_reload: A boolean indicating whether this update is coming from a hot-reload operation (use false by default).
    pub fn update_style_for(
        &mut self,
        uid: i32,
        event: StyleChangeEvent,
        comes_from_hot_reload: bool,
    ) {
        // Update the hot reload state if it exists
        if self.hot_reload_states.is_some() && !comes_from_hot_reload {
            self.register_hot_reload_change(uid, &event);
        }
        let element_op = self.elements.get_mut(&uid);
        if element_op.is_some() {
            let (_, datas) = element_op.unwrap();
            let mut flag_theme = &mut datas.default_theme;
            if event.custom_flag.is_some() {
                let flag = event.custom_flag.unwrap();
                flag_theme = datas
                    .flag_themes
                    .entry(flag.clone())
                    .or_insert(datas.default_theme.clone());
            }
            gen_styles(&event.key, &event.value, flag_theme, &self.fonts);
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
    pub fn pass_event_to_element(
        &mut self,
        target_uid: i32,
        event_type: EventListenerTypes,
        event_datas: EventResponse,
    ) -> QueryResponse {
        let element = self.elements.get_mut(&target_uid);
        if element.is_some() {
            let (element, _) = element.unwrap();
            let element_response_op =
                process_event_callback_for_element(element, &event_type, &event_datas);
            if element_response_op.is_some() {
                return element_response_op.unwrap().response;
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
    pub fn emit_internal_event(
        &mut self,
        uid: i32,
        event: DomInternalMessageType,
        comes_from_hot_reload: bool,
    ) -> QueryResponse {
        let mut event_response: Option<QueryResponse> = None;
        let element = self.elements.get_mut(&uid);
        if element.is_some() {
            let (element, _) = element.unwrap();
            // If the event is a style change, we update the style for the element and don't forward the event to the element itself
            let element_response_op = match event.clone() {
                DomInternalMessageType::StyleChange(event) => {
                    self.update_style_for(uid, event.clone(), comes_from_hot_reload);
                    None
                }
                _ => process_event_for_element(element, event.clone()),
            };

            // Post process the event response, if any, and forward it to the renderer or other elements
            if element_response_op.is_some() {
                event_response = Some(element_response_op.as_ref().unwrap().response.clone());
                // Post-process the event response
                event_response = self.process_element_response(
                    element_response_op.unwrap(),
                    event.clone(),
                    comes_from_hot_reload,
                    event_response,
                );
            }
        }
        if event_response.is_none() {
            return QueryResponse::fail(
                format!(
                    "Element with uid {} not found, or no response from element",
                    uid
                )
                .as_str(),
            );
        }
        return event_response.unwrap();
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
        event: DomInternalMessageType,
        comes_from_hot_reload: bool,
        mut event_response: Option<QueryResponse>,
    ) -> Option<QueryResponse> {
        for renderer_event in element_response.renderer_events {
            match renderer_event {
                RendererEvent::RadioSelectionChange(id, value) => {
                    self.set_radio_selection(id, value);
                }
            }
        }
        for target in element_response.forward_to {
            let new_response =
                self.emit_internal_event(target, event.clone(), comes_from_hot_reload);
            if event_response.is_some() {
                event_response.as_mut().unwrap().concat(new_response);
            }
        }
        return event_response;
    }

    /// Executes a complex query on the elements, handling relationships like child, descendant, sibling, and also.
    ///
    /// Parameters:
    /// - raw_query: The raw complex query string to execute.
    ///
    /// Returns:
    /// - A vector of elements UID that match the complex query.
    pub fn run_complex_query(&self, raw_query: String) -> Vec<i32> {
        let query = ComplexQuery::from(raw_query);
        let firsts = self.raw_element_query(&query.query);
        return self.next_complex(firsts, &Box::new(query));
    }

    /// Recursively processes the next part of a complex query, filtering elements based on their relationships.
    ///
    /// Parameters:
    /// - all: A vector of elements UID that match the current part of the complex query.
    /// - current: A reference to the current ComplexQuery being processed.
    ///
    /// Returns:
    /// - A vector of elements UID that match the entire complex query.
    pub fn next_complex(&self, all: Vec<i32>, current: &Box<ComplexQuery>) -> Vec<i32> {
        if current.next.is_none() {
            return all;
        } else {
            let next_query = current.next.as_ref().unwrap();
            let link = next_query.link_next.as_ref().unwrap();
            let mut next_results: Vec<i32> = Vec::new();
            for element in all.clone() {
                let next_query_results = self.raw_element_query(&next_query.query);
                for next_element in next_query_results {
                    let matches_link = match link {
                        ComplexQueryJoinType::Child => {
                            let parent = self.parent_map.get(&next_element);
                            if parent.is_some() {
                                parent.unwrap() == &element
                            } else {
                                false
                            }
                        }
                        ComplexQueryJoinType::Descendant => {
                            self.is_decendant_of(next_element, element)
                        }
                        ComplexQueryJoinType::Silbling => {
                            let parent_next = self.parent_map.get(&next_element);
                            if parent_next.is_some() {
                                let parent = parent_next.unwrap();
                                let parent_element = self.parent_map.get(&element);
                                if parent_element.is_some() {
                                    parent == parent_element.unwrap()
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        ComplexQueryJoinType::Also => next_element == element,
                    };
                    if matches_link {
                        next_results.push(next_element);
                    }
                }
            }
            let mut ids = self.next_complex(next_results, next_query);
            match link {
                ComplexQueryJoinType::Silbling => ids.append(&mut all.clone()), // For silblings, it's an ADDITION, not a filter, so we keep the previous results. Just like ,
                _ => {}
            };
            return ids;
        }
    }

    /// Recursively checks if a given child element is a descendant of a specified ancestor element.
    ///
    /// Parameters:
    /// - child: The UID of the child element to check.
    /// - ancestor: The UID of the ancestor element to check against.
    ///
    /// Returns:
    /// - A boolean indicating whether the child is a descendant of the ancestor.
    fn is_decendant_of(&self, child: i32, ancestor: i32) -> bool {
        let parent = self.parent_map.get(&child);
        if parent.is_some() {
            if *parent.unwrap() == ancestor {
                return true;
            } else {
                return self.is_decendant_of(*parent.unwrap(), ancestor);
            }
        } else {
            return false;
        }
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
        if self
            .event_listeners
            .iter()
            .any(|e| e.event_type == event_type && e.target == target)
        {
            let event_listener = self
                .event_listeners
                .iter_mut()
                .find(|e| e.event_type == event_type && e.target == target)
                .unwrap();
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

    /// Returns the XmlElement associated with a given UID, if any
    pub fn get_source(&self, uid: i32) -> Option<XmlElement> {
        return self.sources_map.get(&uid).cloned();
    }

    /// Creates an element from its XML representation, handling errors and falling back to a <Void /> element if necessary.
    /// This function ensures that an element is always created, even if the XML representation is invalid.
    ///
    /// Parameters:
    /// - xml_element: The XmlElement to create.
    /// - element_uid: The unique identifier to assign to the new element.
    /// - parent_theme: The XmlTheme of the parent element, if any.
    ///
    /// Returns:
    /// - The UID of the newly created element, void if an error occurred during creation, or 0 if the <Void /> element also failed to create (which should not happen).
    pub fn create_element_safe(
        &mut self,
        xml_element: XmlElement,
        element_uid: i32,
        parent_theme: Option<XmlTheme>,
    ) -> i32 {
        let element_result = generate_element_from_tag(&xml_element, self, element_uid);
        let element;
        if element_result.is_ok() {
            element = element_result.unwrap();
        } else {
            println!(
                "Error during element creation: {}. Generating <Void /> element instead",
                element_result.err().unwrap().to_string()
            );
            let void_element = XmlElement::void();
            let element_result = generate_element_from_tag(&void_element, self, 0);
            if element_result.is_ok() {
                element = element_result.unwrap();
            } else {
                println!(
                    "Error during <Void /> element creation: {}. Returning 0, but the program will probably crash",
                    element_result.err().unwrap().to_string()
                );
                return 0;
            }
        }
        self.init_element(element, Some(xml_element), parent_theme, element_uid);
        return element_uid;
    }

    /// Replace an existing element with a new XmlElement, removing the old element and its children, and creating the new element in its place.
    ///
    /// Parameters:
    /// - element_uid: The unique identifier of the element to replace.
    /// - new_element: The new XmlElement to create in place of the old element.
    ///
    /// Returns:
    /// - A DomQuery that can be used to query the newly created element.
    pub fn replace_element(&mut self, element_uid: i32, new_element: XmlElement) -> DomQuery {
        self.remove_cascade(element_uid, false);
        let element_uid = self.create_element_safe(new_element, element_uid, None);
        return DomQuery {
            query_type: DomQueryType::ByUid(element_uid),
            flag: None,
        };
    }

    /// Removes recursively an element and all its children. It also cleans all the references to the element.
    ///
    /// Parameters:
    /// - element_uid: The unique identifier of the element to remove.
    /// - is_parent: A boolean indicating whether the element is the "root" (used to determine if a <Void /> element should be added to the parent after removal).
    ///
    /// Returns:
    /// - An Option<XmlElement> containing the source of the removed element if it was the root, or None if it was a child element.
    pub fn remove_cascade(&mut self, element_uid: i32, is_parent: bool) -> Option<XmlElement> {
        let mut children = self
            .parent_map
            .iter()
            .filter(|(_, v)| **v == element_uid)
            .map(|(&k, _)| k)
            .collect::<Vec<i32>>();
        if self.virtual_elements.contains_key(&element_uid) {
            let virtual_children = self.virtual_elements.get(&element_uid).unwrap().clone();
            children.append(&mut virtual_children.clone());
        }
        for child in children {
            self.remove_cascade(child, false);
        }
        let old_parent = self.parent_map.get(&element_uid).cloned();
        self.elements.remove(&element_uid);
        self.parent_map.remove(&element_uid);
        self.virtual_elements.remove(&element_uid);
        self.id_map.retain(|_, &mut v| v != element_uid);
        self.classes_map.retain(|_, v| !v.contains(&element_uid));
        self.tags_map.retain(|_, v| !v.contains(&element_uid));
        for event_listener in self.event_listeners.iter_mut() {
            if event_listener.target == element_uid {
                event_listener.handlers.clear();
            }
        }
        let mut source = None;
        if is_parent {
            source = self.sources_map.get(&element_uid).cloned();
        }
        // If the element is a parent, we need to re-add a <Void /> element to the parent, so that the parent can still render correctly
        if is_parent && old_parent.is_some() {
            let xml_element = XmlElement::void();
            self.create_element_safe(xml_element, element_uid, None);
        }
        return source;
    }
}

/// Extracts the custom style flag from a selector flag string, if it exists.
/// The custom style flag is expected to be in the format "for(custom_flag)".
///
/// Parameters:
/// - flag: A reference to the selector flag string.
///
/// Returns:
/// - An Option<String> if the flag is successfully extracted.
pub fn extract_selector_style_flag(flag: &String) -> Option<String> {
    let mut custom_style_for_flag: Option<String> = None;
    let flag = flag;
    if flag.starts_with("for(") && flag.ends_with(")") {
        custom_style_for_flag = Some(
            flag.strip_prefix("for(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .to_string(),
        );
    }
    custom_style_for_flag
}
