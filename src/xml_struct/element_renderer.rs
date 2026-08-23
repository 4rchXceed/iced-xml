use std::collections::HashMap;

use iced::widget::text;

use crate::{
    css_reader::{CssReader, Rule, RuleBlock, Selector},
    dom::{
        events::{ComplexQuery, ComplexQueryJoinType, DomQuery, DomQueryType},
        query::QueryResponse,
    },
    rs_utils::get_unique_id,
    xml_engine::Message,
    xml_struct::{
        elements::{
            library::{
                AnyElement, generate_element_from_tag, process_event_for_element, render_element,
            },
            radio::RadioElement,
        },
        parser::{XmlChangeEvent, XmlElement},
        theming::{Fonts, XmlTheme, gen_styles},
    },
};

#[derive(Clone, Debug)]
pub struct RenderChildDatas {
    pub table_datas: Option<HashMap<String, String>>, // For table elements
}

impl Default for RenderChildDatas {
    fn default() -> Self {
        Self { table_datas: None }
    }
}

pub struct EventListener {
    pub event_type: String,
    pub target: i32,
    pub handlers: Vec<i32>, // List of callbacks to forward the event to
    pub event_uid: i32,
}

pub enum RendererEvent {
    // If elements wants to emit an event to the renderer, it can use this enum
    RadioSelectionChange(String, RadioElement), // (selection_id, selected_element)
}

#[derive(Clone, Debug)]
struct HotReloadStateEntry {
    pub default_theme: XmlTheme,
    pub flag_themes: HashMap<String, XmlTheme>,
}

struct HotReloadState {
    pub rules: Vec<RuleBlock>,
    pub state: HashMap<i32, HotReloadStateEntry>,
}

#[derive(Clone, Debug)]
pub struct ElementExtraData {
    pub default_theme: XmlTheme,
    pub flag_themes: HashMap<String, XmlTheme>,
    pub xml_element: XmlElement,
    pub child_data: Option<RenderChildDatas>,
}

pub struct ElementRenderer {
    pub event_listeners: Vec<EventListener>,
    elements: HashMap<i32, (AnyElement, ElementExtraData)>,
    id_map: HashMap<String, i32>,
    classes_map: HashMap<String, Vec<i32>>,
    tags_map: HashMap<String, Vec<i32>>,
    parent_map: HashMap<i32, i32>, // key: child, value: parent
    virtual_elements: HashMap<i32, Vec<i32>>, // key: parent_uid, value: virtual children uids
    hot_reload_states: Option<HotReloadState>,
    sources_map: HashMap<i32, XmlElement>, // key: source name, value: XmlElement
    // Custom storage for elements
    radio_button_map: HashMap<String, RadioElement>,
    string_map: HashMap<i32, String>,
    fonts: Fonts,
}

impl ElementRenderer {
    pub fn new(fonts: Fonts) -> Self {
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
            // Specific elements for radiobuttons comm
            radio_button_map: HashMap::new(),
            string_map: HashMap::new(),
            fonts: fonts,
        }
    }

    pub fn set_radio_selection(&mut self, id: String, value: RadioElement) {
        self.radio_button_map.insert(id, value);
    }

    pub fn get_radio_selection(&self, id: String) -> Option<RadioElement> {
        return self.radio_button_map.get(&id).cloned();
    }

    pub fn register_stringdb(&mut self, str: String) -> i32 {
        let id = get_unique_id();
        self.string_map.insert(id, str);
        return id;
    }

    pub fn get_stringdb(&self, id: i32) -> Option<String> {
        return self.string_map.get(&id).cloned();
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
        return (true, String::new());
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
                        let query = DomQuery::new(
                            selector.selector_type.clone(),
                            selector.content.clone(),
                            selector.flag.clone(),
                        );
                        let elements: Vec<i32> = self
                            .element_query(&query)
                            .iter()
                            .map(|e| self.post_process_query_result(&query, e.clone()))
                            .flatten()
                            .collect();
                        let mut flag: Option<String> = None;
                        if selector.flag.is_some() {
                            flag = extract_selector_style_flag(&selector.flag.as_ref().unwrap());
                        }
                        // Then we loop through all the elements and revert the style change
                        for element in elements {
                            if flag.is_some() {
                                let old_themes = old_state.state.get(&element);
                                if old_themes.is_some() {
                                    let mut old_flag_theme_op = old_themes
                                        .as_ref()
                                        .unwrap()
                                        .flag_themes
                                        .get(&flag.as_ref().unwrap().to_string())
                                        .cloned();
                                    if old_flag_theme_op.is_none() {
                                        old_flag_theme_op = Some(
                                            old_themes.as_ref().unwrap().default_theme.clone(),
                                        );
                                    }

                                    let mut old_flag_theme = old_flag_theme_op.unwrap();
                                    let real_element = self.elements.get_mut(&element);
                                    if real_element.is_some() {
                                        let rule_to_change = old_flag_theme.clone();
                                        gen_styles(
                                            &rule.name,
                                            &rule.value,
                                            &mut old_flag_theme,
                                            &self.fonts,
                                        );
                                        let (_, datas) = real_element.unwrap();
                                        let mut flag_theme = datas
                                            .flag_themes
                                            .get_mut(&flag.as_ref().unwrap().to_string());
                                        if flag_theme.is_some() {
                                            flag_theme.as_mut().unwrap().apply_only_changes(
                                                &old_flag_theme,
                                                &rule_to_change,
                                                &rule_to_change.clone(),
                                            );
                                        }
                                    }
                                }
                            } else {
                                // get the old theme, before the first css hot-reload-supported change (hot-reload-supported is when hot_reload is true)
                                let old_theme = old_state.state.get(&element);
                                // If we found the old theme, we can revert the style change
                                if old_theme.is_some() {
                                    // Get the real element
                                    let real_element = self.elements.get_mut(&element);
                                    // Then we clone the old theme, remove the style change from a "virtual" theme
                                    let mut rule_to_change =
                                        old_theme.unwrap().default_theme.clone();
                                    gen_styles(
                                        &rule.name,
                                        &rule.value,
                                        &mut rule_to_change,
                                        &self.fonts,
                                    );
                                    if real_element.is_some() {
                                        let (_, datas) = real_element.unwrap();
                                        // And revert the style change by applying only the changes from the old theme to the new theme
                                        datas.default_theme.apply_only_changes(
                                            &old_theme.unwrap().default_theme.clone(),
                                            &rule_to_change,
                                            &old_theme.unwrap().default_theme.clone(),
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
    }

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
        let mut custom_style_for_flag = None;
        if selector.flag.is_some() {
            custom_style_for_flag = extract_selector_style_flag(&selector.flag.as_ref().unwrap());
        }
        for element in elements {
            for rule in rules.iter() {
                self.emit_internal_event(
                    element,
                    XmlChangeEvent::StyleChange(
                        rule.name.clone(),
                        rule.value.clone(),
                        custom_style_for_flag.clone(),
                    ),
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

    pub fn element_query(&self, query: &DomQuery) -> Vec<i32> {
        let query_result = self.raw_element_query(&query.query_type);
        return query_result
            .iter()
            .map(|e| self.post_process_query_result(query, e.clone()))
            .flatten()
            .collect();
    }

    pub fn init_element_from_xml(&mut self, xml_element: &XmlElement, parent_uid: i32) -> i32 {
        // TODO: Add "plugin" support (function provided by the user to resolve custom elements)
        let id = get_unique_id();
        self.parent_map.insert(id, parent_uid);
        let element = generate_element_from_tag(xml_element, self, id);
        if element.is_some() {
            self.init_element(element.unwrap(), Some(xml_element.clone()), None, id);
            return id;
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

    pub fn render_element(
        &self,
        uid: i32,
        child_data: Option<RenderChildDatas>,
    ) -> iced::Element<'_, Message> {
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
                XmlChangeEvent::StyleChange(key, value, custom_flag) => {
                    // Update the hot reload state if it exists
                    if let Some(hot_reload_state) = self.hot_reload_states.as_mut()
                        && !comes_from_hot_reload
                    {
                        if let Some(old_theme) = hot_reload_state.state.get_mut(&uid) {
                            if custom_flag.is_some() {
                                gen_styles(
                                    &key,
                                    &value,
                                    &mut old_theme
                                        .flag_themes
                                        .entry(custom_flag.clone().unwrap())
                                        .or_insert(old_theme.default_theme.clone()),
                                    &self.fonts,
                                );
                            } else {
                                gen_styles(&key, &value, &mut old_theme.default_theme, &self.fonts);
                            }
                        }
                    }
                    let mut flag_theme = &mut datas.default_theme;
                    if custom_flag.is_some() {
                        let flag = custom_flag.unwrap();
                        flag_theme = datas
                            .flag_themes
                            .entry(flag.clone())
                            .or_insert(datas.default_theme.clone());
                    }
                    gen_styles(&key, &value, flag_theme, &self.fonts);
                    None
                }
                _ => process_event_for_element(element, event.clone()),
            };
            if let Some(ev_with_forward) = ev_with_forward {
                for renderer_event in ev_with_forward.2 {
                    match renderer_event {
                        RendererEvent::RadioSelectionChange(id, value) => {
                            self.set_radio_selection(id, value);
                        }
                    }
                }
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

    pub fn run_complex_query(&self, raw_query: String) -> Vec<i32> {
        let query = ComplexQuery::from(raw_query);
        let firsts = self.raw_element_query(&query.query);
        return self.next_complex(firsts, &Box::new(query));
    }

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
                event_uid: get_unique_id(),
            });
        }
    }

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
        let mut source = None;
        if is_parent {
            source = self.sources_map.get(&element_uid).cloned();
        }
        // If the element is a parent, we need to re-add a <Void /> element to the parent, so that the parent can still render correctly
        if is_parent && old_parent.is_some() {
            let xml_element = XmlElement::void();
            let element = generate_element_from_tag(&xml_element, self, element_uid);
            if element.is_some() {
                self.init_element(element.unwrap(), Some(xml_element), None, element_uid);
            } else {
                panic!("Block: <Void /> doesn't exists");
            }
        }
        return source;
    }
}

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
