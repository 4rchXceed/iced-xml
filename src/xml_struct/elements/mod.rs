pub mod button;
pub mod center;
pub mod checkbox;
pub mod container;
pub mod element_base;
pub mod label;
pub mod row;
// Library
pub mod library;

use std::collections::HashMap;

use iced::widget::text;

use crate::{
    css_reader::{CssReader, Rule, RuleBlock, Selector},
    dom::{
        events::{DomQuery, DomQueryType},
        query::QueryResponse,
    },
    xml_engine::Message,
    xml_struct::{
        elements::library::{
            AnyElement, generate_element_from_tag, process_event_for_element, render_element,
        },
        parser::{XmlChangeEvent, XmlElement},
        theming::{XmlTheme, gen_styles},
    },
};

pub struct EventListener {
    pub event_type: String,
    pub target: i32,
    pub handlers: Vec<i32>, // List of callbacks to forward the event to
    pub event_uid: i32,
}

struct HotReloadState {
    pub rules: Vec<RuleBlock>,
    pub state: HashMap<i32, XmlTheme>,
}

struct ElementExtraData {
    pub theme: XmlTheme,
    pub xml_element: XmlElement,
}

pub struct ElementRenderer {
    pub event_listeners: Vec<EventListener>,
    elements: HashMap<i32, (AnyElement, ElementExtraData)>,
    id_map: HashMap<String, i32>,
    classes_map: HashMap<String, Vec<i32>>,
    tags_map: HashMap<String, Vec<i32>>,
    virtual_elements: HashMap<i32, Vec<i32>>, // key: parent_uid, value: virtual children uids
    last_uid: i32,
    hot_reload_states: Option<HotReloadState>,
}

impl ElementRenderer {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            last_uid: 0,
            id_map: HashMap::new(),
            classes_map: HashMap::new(),
            tags_map: HashMap::new(),
            event_listeners: Vec::new(),
            hot_reload_states: None,
            virtual_elements: HashMap::new(),
        }
    }

    // TODO: Move all hot-reload logic into a separate file
    pub fn load_css(&mut self, css: &str, hot_reload: bool) -> (bool, String) {
        let mut reader = CssReader::new(css);
        reader.parse();
        if reader.kill_switch {
            return (false, reader.kill_message);
        }
        if hot_reload {
            if self.hot_reload_states.is_some() {
                self.update_state(&reader.rules);
                self.cleanup_for_hot_reload(&reader.rules);
            } else {
                self.hot_reload_states = Some(self.generate_state(&reader.rules));
            }
        }
        for rule_block in &reader.rules {
            // TODO: Add support for multiple selectors in a single rule block
            let selectors = &rule_block.selectors;
            for selector in selectors {
                self.apply_rules(selector, &rule_block.rules, hot_reload);
            }
        }
        (true, String::new())
    }

    pub fn get_data(&mut self, element: i32, key: &str) -> Option<String> {
        let element = self.elements.get_mut(&element);
        if element.is_some() {
            let (_, data) = element.unwrap();
            data.xml_element.datas.get(key).cloned()
        } else {
            None
        }
    }

    fn cleanup_for_hot_reload(&mut self, new_rules: &Vec<RuleBlock>) {
        let old_state = self.hot_reload_states.as_ref().unwrap();
        let mut state_hashed: HashMap<String, (Selector, Rule)> = HashMap::new();
        for rule_block in new_rules.iter() {
            for selector in &rule_block.selectors {
                for rule in &rule_block.rules {
                    state_hashed.insert(
                        rule.hash_with_selector(selector),
                        (selector.clone(), rule.clone()),
                    );
                }
            }
        }
        // Ok, small explaination of what's going on here
        for old_rule_block in old_state.rules.iter() {
            for selector in &old_rule_block.selectors {
                for rule in &old_rule_block.rules {
                    let hash = rule.hash_with_selector(selector);
                    // Here: rule has been removed, we need to revert the style change
                    if !state_hashed.contains_key(&hash) {
                        // We are selecting the correct element, so we can work on it
                        let elements = self.element_query(&DomQuery::new(
                            selector.selector_type.clone(),
                            selector.content.clone(),
                            selector.flag.clone(),
                        ));
                        // Then we loop through all the elements and revert the style change
                        for element in elements {
                            // get the old theme, before the first css hot-reload-supported change (hot-reload-supported is when hot_reload is true)
                            let old_theme = old_state.state.get(&element);
                            // If we found the old theme, we can revert the style change
                            if old_theme.is_some() {
                                // Get the real element
                                let real_element = self.elements.get_mut(&element);
                                // Then we clone the old theme, remove the style change from a "virtual" theme
                                let mut rule_to_change = old_theme.unwrap().clone();
                                gen_styles(&rule.name, &rule.value, &mut rule_to_change);
                                if real_element.is_some() {
                                    let (_, datas) = real_element.unwrap();
                                    // And revert the style change by applying only the changes from the old theme to the new theme
                                    datas.theme.apply_only_changes(
                                        &old_theme.unwrap().clone(),
                                        &rule_to_change,
                                        &old_theme.unwrap().clone(),
                                    );
                                    // By doing all of the gen_styles and other stuff, we avoid having the create a revert function (from XmlTheme to rules)
                                    // We still need apply_only_changes tho
                                }
                            } // Normally, we should always find the old theme, but if we don't, we just skip it
                        }
                    }
                }
            }
        }
    }

    fn generate_state(&mut self, rules: &Vec<RuleBlock>) -> HotReloadState {
        let mut state: HashMap<i32, XmlTheme> = HashMap::new();
        for (uid, (_, datas)) in &self.elements {
            state.insert(*uid, datas.theme.clone());
        }
        HotReloadState {
            rules: rules.clone(),
            state: state,
        }
    }

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
        for element in elements {
            for rule in rules.iter() {
                // HERE
                self.emit_internal_event(
                    element,
                    XmlChangeEvent::StyleChange(rule.name.clone(), rule.value.clone()),
                    comes_from_hot_reload,
                );
            }
        }
    }

    pub fn get_element(&mut self, uid: i32) -> &mut AnyElement {
        let element = self.elements.get_mut(&uid);
        if element.is_some() {
            return &mut element.unwrap().0;
        } else {
            panic!(
                "Element not found: {}, but called with a no-fail method. Probably a program state issue",
                uid
            )
        }
    }

    fn post_process_query_result(&self, query: &DomQuery, element: i32) -> Vec<i32> {
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
                    println!("Unknown selector flag: {}", flag);
                    vec![]
                }
            };
        } else {
            return vec![element];
        }
    }

    pub fn element_query(&self, query: &DomQuery) -> Vec<i32> {
        let query_result = match &query.query_type {
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
        };
        return query_result
            .iter()
            .map(|e| self.post_process_query_result(query, e.clone()))
            .flatten()
            .collect();
    }

    pub fn init_element_from_xml(&mut self, xml_element: &XmlElement) -> i32 {
        // TODO: Add "plugin" support (function provided by the user to resolve custom elements)
        self.last_uid += 1;
        let uid = self.last_uid; // "Backup"
        let element = generate_element_from_tag(xml_element, self, uid);
        if let Some(element) = element {
            self.init_element(element, Some(xml_element.clone()), None, uid);
            return uid;
        } else {
            panic!("Block: <{} /> doesn't exists", &xml_element.tag);
        }
    }

    pub fn init_element_virt(
        &mut self,
        element: AnyElement,
        parent_theme: Option<XmlTheme>,
        parent_uid: i32,
    ) -> i32 {
        self.last_uid += 1;
        let uid = self.last_uid; // "Backup"
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
        self.elements.insert(
            uid,
            (
                element,
                ElementExtraData {
                    theme: xml_element.theme.clone(),
                    xml_element: xml_element,
                },
            ),
        );
    }

    pub fn render_element(&self, uid: i32) -> iced::Element<'_, Message> {
        let element = self.elements.get(&uid);
        if element.is_some() {
            let events = self
                .event_listeners
                .iter()
                .filter(|v| v.target == uid)
                .collect::<Vec<&EventListener>>();
            let (element, datas) = element.unwrap();
            let output = render_element(element, self, &datas.theme, events, uid);
            output
        } else {
            return text(format!("Element with id {} not found", uid)).into();
        }
    }

    pub fn emit_internal_event(
        &mut self,
        uid: i32,
        event: XmlChangeEvent,
        comes_from_hot_reload: bool,
    ) -> QueryResponse {
        let mut event_response: Option<QueryResponse> = None;
        let element = self.elements.get_mut(&uid);
        if element.is_some() {
            let (element, datas) = element.unwrap();
            let ev_with_forward = match event.clone() {
                XmlChangeEvent::StyleChange(key, value) => {
                    // Update the hot reload state if it exists
                    if let Some(hot_reload_state) = self.hot_reload_states.as_mut()
                        && !comes_from_hot_reload
                    {
                        if let Some(old_theme) = hot_reload_state.state.get_mut(&uid) {
                            gen_styles(&key, &value, old_theme);
                        }
                    }
                    gen_styles(&key, &value, &mut datas.theme);
                    None
                }
                _ => process_event_for_element(element, event.clone()),
            };
            if let Some(ev_with_forward) = ev_with_forward {
                for target in ev_with_forward.1 {
                    self.emit_internal_event(target, event.clone(), comes_from_hot_reload);
                }
                event_response = Some(ev_with_forward.0);
            }
        }
        if event_response.is_none() {
            return QueryResponse::new(false);
        }
        event_response.unwrap()
    }

    pub fn register_event(&mut self, event_type: String, target: i32, handler: i32) {
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
                event_uid: self.last_uid,
            });
            self.last_uid += 1;
        }
    }
}
