use log::error;

use std::collections::HashMap;

use crate::css::errors::CssParseError;
use crate::dom::query::{ComplexQuery, ComplexQueryJoinType, DomQuery, DomQueryType};
use crate::rs_utils::get_unique_id;
use crate::xml_struct::elements::library::{AnyElement, generate_element_from_tag};
use crate::xml_struct::elements::radio::RadioElement;
use crate::xml_struct::theming::XmlTheme;
use crate::xml_struct::xml_element::XmlElement;

use super::elements_renderer::ElementsRenderer;
use super::utils::{ElementExtraData, EventListener};

//// Stores the mappings of elements
pub struct ElementMappings {
    /// The elements stored in the renderer, with their extra data (see ElementExtraData)
    /// TODO: Convert to a HashMap<uid, ElementDatas> to avoid the tuple
    pub elements: HashMap<i32, (AnyElement, ElementExtraData)>,
    /// Maps the element's ID to its UID, for quick access
    pub id_map: HashMap<String, i32>,
    /// Maps the element's class to its UIDs, for quick access
    pub classes_map: HashMap<String, Vec<i32>>,
    /// Maps the element's tag to its UIDs, for quick access
    pub tags_map: HashMap<String, Vec<i32>>,
    /// Maps the element's UID to its parent's UID, for quick access
    pub parent_map: HashMap<i32, i32>, // key: child, value: parent
    /// Maps the element's UID to its XML source, for quick access
    pub sources_map: HashMap<i32, Box<XmlElement>>, // key: source name, value: XmlElement
    /// Maps the element's UID to its virtual children UIDs, for quick access
    /// Virtual childrens are elements that are not declared in the XML, but are created by an element.
    /// I'm using this system, so the sub-elements are still stylable by CSS
    /// TODO: Add a better system, or remove it entirely. I'll probably use the flag system
    pub virtual_elements: HashMap<i32, Vec<i32>>, // key: parent_uid, value: virtual children uids
    // Custom storage for elements
    /// Stores the selected radio button for each radio group, by ID
    radio_button_map: HashMap<String, RadioElement>,
    /// This one's tricky: if an element needs to store a string, but it needs to have the Copy trait, it can use this
    string_map: HashMap<i32, String>,
}

impl ElementMappings {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            id_map: HashMap::new(),
            classes_map: HashMap::new(),
            tags_map: HashMap::new(),
            parent_map: HashMap::new(),
            sources_map: HashMap::new(),
            virtual_elements: HashMap::new(),
            // Specific elements for radiobuttons comm
            radio_button_map: HashMap::new(),
            string_map: HashMap::new(),
        }
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

    /// "Post-init" an element, after it has been created, to set its XML representation and theme.
    pub fn add_element(
        &mut self,
        element: AnyElement,
        xml: Option<Box<XmlElement>>,
        parent_theme: Option<XmlTheme>,
        uid: i32,
    ) {
        let xml_element = if let Some(xml) = xml {
            xml
        } else {
            let mut xml_element = Box::new(XmlElement::virt());

            if let Some(parent_theme) = parent_theme {
                xml_element.theme = parent_theme;
            }

            xml_element
        };

        // Save the ID
        if let Some(id) = &xml_element.id {
            self.id_map.insert(id.clone(), uid);
        }

        // Save the classes
        for class in &xml_element.classes {
            if let Some(class_elements) = self.classes_map.get_mut(class) {
                class_elements.push(uid);
            } else {
                self.classes_map.insert(class.clone(), vec![uid]);
            }
        }

        // Save the tag
        if let Some(tag_elements) = self.tags_map.get_mut(&xml_element.tag) {
            tag_elements.push(uid);
        } else {
            self.tags_map.insert(xml_element.tag.clone(), vec![uid]);
        }

        // Save the source
        self.sources_map.insert(uid, xml_element.clone());

        // Save the element and its extra data
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

    /// Executes a complex query on the elements, handling relationships like child, descendant, sibling, and also.
    ///
    /// Parameters:
    /// - raw_query: The raw complex query string to execute.
    ///
    /// Returns:
    /// - A vector of elements UID that match the complex query.
    pub fn run_complex_query(&self, raw_query: String) -> Result<Vec<i32>, CssParseError> {
        let query = ComplexQuery::from(raw_query)?;

        let firsts = self.raw_element_query(&query.query);

        return Ok(self.next_complex(firsts, &Box::new(query)));
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
        if let Some(next_query) = &current.next
            && let Some(link) = &next_query.link_next
        {
            let mut next_results: Vec<i32> = Vec::new();

            for element in &all {
                let next_query_results = self.raw_element_query(&next_query.query);

                for next_query_element in next_query_results {
                    let matches_link = match link {
                        ComplexQueryJoinType::Child => {
                            let next_query_element_parent =
                                self.parent_map.get(&next_query_element);

                            if let Some(parent_of_next_element) = next_query_element_parent
                                && parent_of_next_element == element
                            {
                                true
                            } else {
                                false
                            }
                        }
                        ComplexQueryJoinType::Descendant => {
                            self.is_decendant_of(next_query_element, *element)
                        }
                        ComplexQueryJoinType::Silbling => {
                            let parent_of_next_element_op =
                                self.parent_map.get(&next_query_element);

                            if let Some(parent_of_next_element) = parent_of_next_element_op {
                                let parent_of_current_element = self.parent_map.get(&element);

                                if let Some(parent_of_current_element) = parent_of_current_element {
                                    parent_of_next_element == parent_of_current_element
                                        && next_query_element != *element
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        ComplexQueryJoinType::Also => &next_query_element == element,
                    };

                    if matches_link {
                        next_results.push(next_query_element);
                    }
                }
            }

            // Forward
            let mut ids = self.next_complex(next_results, next_query);

            // For silblings, it's an ADDITION, not a filter, so we keep the previous results.
            match link {
                ComplexQueryJoinType::Silbling => ids.append(&mut all.clone()),
                _ => {}
            };

            return ids;
        } else {
            return all;
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
    pub fn post_process_query_result(&self, query: &DomQuery, element: i32) -> Vec<i32> {
        if let Some(flag) = &query.flag {
            return match flag.as_str() {
                "virtuals" => {
                    let virtual_children = self.virtual_elements.get(&element);

                    if let Some(virtual_children) = virtual_children {
                        virtual_children.clone()
                    } else {
                        vec![]
                    }
                }
                _ => {
                    if ElementsRenderer::extract_selector_style_flag(flag).is_some() {
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
                match self.run_complex_query(raw_complex_query.clone()) {
                    Err(e) => {
                        error!("Invalid complex query: {e}");
                        return Vec::new();
                    }
                    Ok(uids) => {
                        return uids;
                    }
                }
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

        if let Some(parent) = parent {
            if *parent == ancestor {
                return true;
            } else {
                return self.is_decendant_of(*parent, ancestor);
            }
        } else {
            return false;
        }
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
        xml_element: Box<XmlElement>,
        element_uid: i32,
        parent_theme: Option<XmlTheme>,
    ) -> i32 {
        let element_result = generate_element_from_tag(xml_element.clone(), self, element_uid);

        let element = if let Ok(element_result) = element_result {
            element_result
        } else {
            // An error occurred during element creation
            error!(
                "Error during element creation: {}. Generating <Void /> element instead",
                element_result.err().unwrap().to_string()
            );

            let void_element = Box::new(XmlElement::void());

            let element_result = generate_element_from_tag(void_element, self, 0);

            match element_result {
                Ok(result) => result,
                Err(e) => {
                    error!(
                        "Error during <Void /> element creation: {}. Returning 0, but the program will probably crash",
                        e
                    );
                    return 0;
                }
            }
        };

        self.add_element(element, Some(xml_element), parent_theme, element_uid);

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
    pub fn replace_element(
        &mut self,
        element_uid: i32,
        new_element: Box<XmlElement>,
        event_listeners: &mut Vec<EventListener>,
    ) -> DomQuery {
        self.remove_cascade(element_uid, false, event_listeners);

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
    pub fn remove_cascade(
        &mut self,
        element_uid: i32,
        is_parent: bool,
        event_listeners: &mut Vec<EventListener>,
    ) -> Option<Box<XmlElement>> {
        // Gets all children of the element, both real and virtual, and removes them recursively
        let mut children = self
            .parent_map
            .iter()
            .filter(|(_, v)| **v == element_uid)
            .map(|(&k, _)| k)
            .collect::<Vec<i32>>();

        if let Some(mut virtual_children) = self.virtual_elements.get_mut(&element_uid) {
            children.append(&mut virtual_children);
        }

        // Remove them
        for child in children {
            self.remove_cascade(child, false, event_listeners);
        }

        // Remove all references to the element
        self.elements.remove(&element_uid);
        self.virtual_elements.remove(&element_uid);
        self.id_map.retain(|_, &mut v| v != element_uid);
        self.classes_map.retain(|_, v| !v.contains(&element_uid));
        self.tags_map.retain(|_, v| !v.contains(&element_uid));

        for event_listener in event_listeners {
            if event_listener.target == element_uid {
                event_listener.handlers.clear();
            }
        }

        let mut source = None;

        if is_parent {
            source = self.sources_map.get(&element_uid).cloned();
        }

        // If the element is a parent, we need to re-add a <Void /> element to the parent, so that the parent can still render correctly
        if is_parent && self.parent_map.contains_key(&element_uid) {
            let xml_element = Box::new(XmlElement::void());

            self.create_element_safe(xml_element, element_uid, None);
        }

        return source;
    }

    /// Returns the XmlElement associated with a given UID, if any
    pub fn get_source(&self, uid: i32) -> Option<Box<XmlElement>> {
        return self.sources_map.get(&uid).cloned();
    }

    /// Initializes an element from its XML representation and associates it with a parent element.
    ///
    /// Parameters:
    /// - xml_element: The XmlElement to initialize.
    /// - parent_uid: The UID of the parent element.
    ///
    /// Returns:
    /// - The UID of the newly created element.
    pub fn init_element_from_xml(&mut self, xml_element: Box<XmlElement>, parent_uid: i32) -> i32 {
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
        let uid = get_unique_id();

        self.add_element(element, None, parent_theme, uid);

        let parent_virtual_children = self.virtual_elements.get_mut(&parent_uid);

        if let Some(virtual_elements) = parent_virtual_children {
            virtual_elements.push(uid);
        } else {
            self.virtual_elements.insert(parent_uid, vec![uid]);
        }

        return uid;
    }
}
