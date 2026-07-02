use std::collections::HashMap;

use iced::widget::text;

use crate::{
    css_reader::{CssReader, Rule, RuleBlock, Selector},
    dom::{events::DomQuery, query::QueryResponse},
    logger::fatal,
    xml_engine::Message,
    xml_struct::{
        elements::{
            button::Button, center::Center, container::Container, element_base::ElementBase,
            label::Label, row::Row,
        },
        parser::{XmlChangeEvent, XmlElement, XmlTheme, gen_styles},
    },
};

pub mod button;
pub mod center;
pub mod container;
pub mod element_base;
pub mod label;
pub mod row;

pub enum AnyElement {
    Label(Label),
    Container(Container),
    Button(Button),
    Row(Row),
    Center(Center),
}

pub struct EventListener {
    pub event_type: String,
    pub target: i32,
    pub handler: i32,
    pub event_uid: i32,
}

struct HotReloadState {
    pub rules: Vec<RuleBlock>,
    pub state: HashMap<i32, XmlTheme>,
}

pub struct ElementRenderer {
    pub elements: HashMap<i32, (AnyElement, XmlTheme, XmlElement)>,
    pub id_map: HashMap<String, i32>,
    pub classes_map: HashMap<String, Vec<i32>>,
    pub tags_map: HashMap<String, Vec<i32>>,
    pub last_uid: i32,
    pub event_listeners: Vec<EventListener>,
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
            let (_, _, xml_element) = element.unwrap();
            xml_element.datas.get(key).cloned()
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
                        ));
                        if elements.is_some() {
                            // Then we loop through all the elements and revert the style change
                            for element in elements.unwrap() {
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
                                        let (_, theme, _) = real_element.unwrap();
                                        // And revert the style change by applying only the changes from the old theme to the new theme
                                        theme.apply_only_changes(
                                            &old_theme.unwrap().clone(),
                                            &rule_to_change,
                                            &old_theme.unwrap().clone(),
                                        );
                                        // By doing all of the gen_styles and other stuff, we avoid having the create a revert function (from XmlTheme to rules)
                                        // We still need apply_only_changes tho
                                    }
                                } // Normally, we should always find the old theme, but if we don't, we just skip it
                            }
                        } // else: invalid selector
                    }
                }
            }
        }
    }

    fn generate_state(&mut self, rules: &Vec<RuleBlock>) -> HotReloadState {
        let mut state: HashMap<i32, XmlTheme> = HashMap::new();
        for (uid, (_, theme, _)) in &self.elements {
            state.insert(*uid, theme.clone());
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
        let dom_query = DomQuery::new(selector.selector_type.clone(), selector.content.clone());

        let elements = self.element_query(&dom_query);
        if elements.is_some() {
            for element in elements.unwrap() {
                for rule in rules.iter() {
                    self.emit_internal_event(
                        element,
                        XmlChangeEvent::StyleChange(rule.name.clone(), rule.value.clone()),
                        comes_from_hot_reload,
                    );
                }
            }
        }
    }

    pub fn element_query(&self, query: &DomQuery) -> Option<Vec<i32>> {
        return match query {
            DomQuery::ById(id) => {
                if let Some(uid) = self.id_map.get(id) {
                    Some(vec![*uid])
                } else {
                    None
                }
            }
            DomQuery::ByUid(uid) => {
                if self.elements.contains_key(&uid) {
                    Some(vec![uid.clone()])
                } else {
                    None
                }
            } // _ => None,
            DomQuery::Class(class) => {
                if let Some(uids) = self.classes_map.get(class) {
                    Some(uids.clone())
                } else {
                    None
                }
            }
            DomQuery::Tag(tag) => {
                if let Some(uids) = self.tags_map.get(tag) {
                    Some(uids.clone())
                } else {
                    None
                }
            }
            DomQuery::All => Some(self.elements.keys().cloned().collect()),
            DomQuery::Unused => None,
        };
    }

    pub fn init_element(&mut self, xml_element: &XmlElement) -> i32 {
        // TODO: Add "plugin" support (function provided by the user to resolve custom elements)
        let element: Option<AnyElement> = match xml_element.tag.as_str() {
            "Label" => Some(AnyElement::Label(Label::new(xml_element, self))),
            "Div" => Some(AnyElement::Container(Container::new(xml_element, self))),
            "Col" => Some(AnyElement::Container(Container::new(xml_element, self))),
            "Row" => Some(AnyElement::Row(Row::new(xml_element, self))),
            "Window" => Some(AnyElement::Container(Container::new(xml_element, self))), // Window works the same way as a container (FOR NOW), we'll use the same logic
            "Button" => Some(AnyElement::Button(Button::new(xml_element, self))),
            "Center" => Some(AnyElement::Center(Center::new(xml_element, self))),
            _ => None,
        };
        if let Some(element) = element {
            if xml_element.id.is_some() {
                self.id_map
                    .insert(xml_element.id.clone().unwrap().to_string(), self.last_uid);
            }
            for class in &xml_element.classes {
                let class_map = self.classes_map.get(class);
                if class_map.is_some() {
                    self.classes_map.get_mut(class).unwrap().push(self.last_uid);
                } else {
                    self.classes_map.insert(class.clone(), vec![self.last_uid]);
                }
            }
            let tag_map = self.tags_map.get(&xml_element.tag);
            if tag_map.is_some() {
                self.tags_map
                    .get_mut(&xml_element.tag)
                    .unwrap()
                    .push(self.last_uid);
            } else {
                self.tags_map
                    .insert(xml_element.tag.clone(), vec![self.last_uid]);
            }
            self.elements.insert(
                self.last_uid,
                (element, xml_element.theme.clone(), xml_element.clone()),
            );
            self.last_uid += 1;
            return self.last_uid - 1;
        } else {
            fatal(format!("Block: <{} /> doesn't exists", &xml_element.tag).as_str());
            return -1;
        }
    }

    pub fn render_element(&self, uid: i32) -> iced::Element<'_, Message> {
        let element = self.elements.get(&uid);
        if element.is_some() {
            let events = self
                .event_listeners
                .iter()
                .filter(|v| v.target == uid)
                .collect::<Vec<&EventListener>>();
            let (element, theme, _) = element.unwrap();
            let output = match element {
                AnyElement::Label(label) => label.render(self, theme, events, uid),
                AnyElement::Container(container) => container.render(self, theme, events, uid),
                AnyElement::Button(button) => button.render(self, theme, events, uid),
                AnyElement::Row(row) => row.render(self, theme, events, uid),
                AnyElement::Center(center) => center.render(self, theme, events, uid),
            };
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
            let (element, theme, _) = element.unwrap();
            event_response = match event {
                XmlChangeEvent::StyleChange(key, value) => {
                    // Update the hot reload state if it exists
                    if let Some(hot_reload_state) = self.hot_reload_states.as_mut()
                        && !comes_from_hot_reload
                    {
                        if let Some(old_theme) = hot_reload_state.state.get_mut(&uid) {
                            gen_styles(&key, &value, old_theme);
                        }
                    }
                    gen_styles(&key, &value, theme);
                    None
                }
                _ => match element {
                    AnyElement::Label(label) => label.process_event(&event),
                    AnyElement::Container(container) => container.process_event(&event),
                    AnyElement::Button(button) => button.process_event(&event),
                    AnyElement::Row(row) => row.process_event(&event),
                    AnyElement::Center(center) => center.process_event(&event),
                },
            };
        }
        if event_response.is_none() {
            return QueryResponse::new(false);
        }
        event_response.unwrap()
    }

    pub fn register_event(&mut self, event_type: String, target: i32, handler: i32) {
        self.event_listeners.push(EventListener {
            event_type: event_type,
            target: target,
            handler: handler,
            event_uid: self.last_uid,
        });
        self.last_uid += 1;
    }
}
