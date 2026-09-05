//! This file contains all the utility functions that are related to the Rust code, such as hashing, unique IDs, and other utilities.
use std::{cell::Cell, collections::HashMap, f32::consts::PI};

use crate::xml_struct::parser::XmlElement;

// Unique ID for all windows:
// Thread-safe, since we ALWAYS work in the main thread (for the UI)
thread_local! {
    static CURRENT_UID: Cell<i32> = Cell::new(1);
}

///  Safe read file function, returns a Result<String, std::io::Error> instead of panicking on error
///
///  Parameters:
///
///  - filename: the filename to read
///
///  Returns:
///
///  - the file content or an error
pub fn safe_read_file(filename: &str) -> Result<String, std::io::Error> {
    // Read the file
    let content = std::fs::read_to_string(filename);
    // Then process the result.
    if content.is_err() {
        return Err(content.unwrap_err());
    }
    return Ok(content.unwrap());
}

///  Get a unique ID for all purposes, such as element IDs, component IDs, etc.
///
///  No parameters.
///
///  Returns:
///
///  - a unique ID (i32)
///
///  Note: UIDs are unique ONLY within the same thread, so if you use multiple threads, you need to manage UIDs yourself.
pub fn get_unique_id() -> i32 {
    CURRENT_UID.with(|id| {
        // Get
        let current_id = id.get();
        // Set
        id.set(current_id + 1);
        // Return
        return current_id;
    })
}

// Hash types that aren't natively hashable

///  A wrapper for f32 that implements Hash, so it can be used in iced's Message enum, which requires all types to implement Hash.
///
///  Usage: HashableF32::new(1.0)
///
///  To get the value: HashableF32.value()
#[derive(Debug, Clone)]
pub struct HashableF32(f32);

impl HashableF32 {
    ///  Create a new HashableF32 from a f32 value
    ///
    ///  Parameters:
    ///
    ///  - value: the f32 value to wrap
    ///
    ///  Returns:
    ///
    ///  - a new HashableF32
    pub fn new(value: f32) -> Self {
        return HashableF32(value);
    }

    ///  Get the f32 value from the HashableF32
    ///
    ///  No parameters.
    ///
    ///  Returns:
    ///
    ///  - the f32 value
    pub fn value(&self) -> f32 {
        return self.0;
    }
}

impl std::hash::Hash for HashableF32 {
    ///  Hash the HashableF32 by converting it to bits and hashing the bits
    ///
    ///  Parameters:
    ///
    ///  - state: the hasher to use
    ///
    ///  Returns:
    ///
    ///  - void
    ///
    ///  Note: the hasher is updated
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

///  A wrapper for the iced's PaneGrid's Target that implements Hash, so it can be used in iced's Message enum, which requires all types to implement Hash.
///
///  Usage: HashableGridTarget::new(...)
///
///  To get the value: HashableGridTarget.value()
#[derive(Debug, Clone)]
pub struct HashableGridTarget(iced::widget::pane_grid::Target);

impl HashableGridTarget {
    ///  Create a new HashableGridTarget from a Panegrid Target value (iced::widget::pane_grid::Target)
    ///
    ///  Parameters:
    ///
    ///  - value: the Panegrid Target value to wrap
    ///
    ///  Returns:
    ///
    ///  - a new HashableGridTarget
    pub fn new(value: iced::widget::pane_grid::Target) -> Self {
        HashableGridTarget(value)
    }

    ///  Get the Panegrid Target value from the HashableGridTarget
    ///
    ///  No parameters.
    ///
    ///  Returns:
    ///
    ///  - the Panegrid Target value
    pub fn value(&self) -> iced::widget::pane_grid::Target {
        return self.0;
    }

    ///  Util to hash the iced::widget::pane_grid::Edge enum, since it doesn't implement Hash
    ///
    ///  Parameters:
    ///
    ///  - edge: the Edge to hash
    ///
    ///  Returns:
    ///
    ///  - a u64 hash of the Edge
    fn hash_edge(edge: &iced::widget::pane_grid::Edge) -> u64 {
        match edge {
            iced::widget::pane_grid::Edge::Bottom => 0,
            iced::widget::pane_grid::Edge::Left => 1,
            iced::widget::pane_grid::Edge::Right => 2,
            iced::widget::pane_grid::Edge::Top => 3,
        }
    }
}

impl std::hash::Hash for HashableGridTarget {
    ///  Hash the HashableGridTarget by hashing the underlying iced::widget::pane_grid::Target
    ///
    ///  Parameters:
    ///
    ///  - state: the hasher to use
    ///
    ///  Returns:
    ///
    ///  - void
    ///
    ///  Note: the hasher is updated
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert everything to a u64
        let first = match self.0 {
            // If it's an edge, hash the edge and add 1 to it, so we can differentiate between edges and panes
            // Possible values: 1, 2, 3, 4 (for the edges)
            iced::widget::pane_grid::Target::Edge(edge) => {
                let edge_hash = HashableGridTarget::hash_edge(&edge);
                1 + edge_hash
            }
            // If it's a pane, hash the pane and the region, and add 5 to it, so we can differentiate between edges and panes
            // Possible values: 5, 6, 7, 8, 9
            iced::widget::pane_grid::Target::Pane(pane, region) => {
                // Hash the pane and the region
                pane.hash(state);
                let region_hash = match region {
                    iced::widget::pane_grid::Region::Center => 0,
                    iced::widget::pane_grid::Region::Edge(edge) => {
                        HashableGridTarget::hash_edge(&edge)
                    }
                };
                5 + region_hash
            }
        };
        // ...then hash the u64
        first.hash(state);
    }
}

///  Even tho is sounds akward, HashMaps are not hashable, so we need to wrap them in a struct that implements Hash, so we can use them in iced's Message enum, which requires all types to implement Hash.
///
///  Usage: HashableHashMap::new(...)
///
///  To get the value: HashableHashMap.value()
#[derive(Debug, Clone)]
pub struct HashableHashMap<K, V>(pub HashMap<K, V>);

impl<K, V> HashableHashMap<K, V> {
    ///  Create a new HashableHashMap from a HashMap value (HashMap)
    ///
    ///  Parameters:
    ///
    ///  - map: the HashMap value to wrap
    ///
    ///  Returns:
    ///
    ///  - a new HashableHashMap
    pub fn new(map: HashMap<K, V>) -> Self {
        return Self(map);
    }

    ///  Get the HashMap value from the HashableHashMap
    ///
    ///  No parameters.
    ///
    ///  Returns:
    ///
    ///  - the HashMap value
    pub fn value(&self) -> &HashMap<K, V> {
        return &self.0;
    }
}

impl<K: std::hash::Hash, V: std::hash::Hash> std::hash::Hash for HashableHashMap<K, V> {
    ///  Hash the HashableHashMap by hashing the underlying HashMap
    ///
    ///  Parameters:
    ///
    ///  - state: the hasher to use
    ///
    ///  Returns:
    ///
    ///  - void
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Get the items in the HashMap, sort them by key, and then hash them in order
        let mut items: Vec<(&K, &V)> = self.0.iter().collect();
        items.sort_by(|a, b| a.0.hash(state).cmp(&b.0.hash(state)));
        // Then hash the items in order
        for (k, v) in items {
            k.hash(state);
            v.hash(state);
        }
    }
}

///  A wrapper for the iced's Textarea Edit that implements Hash, so it can be used in iced's Message enum, which requires all types to implement Hash.
///
///  Usage: HashableTextareaEdit::new(...)
///
///  To get the value: HashableTextareaEdit.value()
#[derive(Debug, Clone)]
pub struct HashableTextareaEdit(iced::widget::text_editor::Edit);

impl HashableTextareaEdit {
    /// Create a new HashableTextareaEdit from a Textarea Edit value (iced::widget::text_editor::Edit)
    ///
    /// Parameters:
    ///
    ///  - value: the Textarea Edit value to wrap
    ///
    /// Returns:
    ///
    /// - a new HashableTextareaEdit
    pub fn new(value: iced::widget::text_editor::Edit) -> Self {
        HashableTextareaEdit(value)
    }

    /// Get the Textarea Edit value from the HashableTextareaEdit
    ///
    /// No parameters.
    ///
    /// Returns:
    ///
    /// - the Textarea Edit value
    pub fn value(&self) -> &iced::widget::text_editor::Edit {
        return &self.0;
    }
}

impl std::hash::Hash for HashableTextareaEdit {
    /// Hash the HashableTextareaEdit by hashing the underlying iced::widget::text_editor::Edit
    ///
    /// Parameters:
    ///
    /// - state: the hasher to use
    ///
    /// Returns:
    ///
    /// - void
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the underlying iced::widget::text_editor::Edit by matching on its variant and hashing the relevant data
        match &self.0 {
            iced::widget::text_editor::Edit::Backspace => {
                0.hash(state);
            }
            iced::widget::text_editor::Edit::Delete => {
                1.hash(state);
            }
            iced::widget::text_editor::Edit::Enter => {
                2.hash(state);
            }
            iced::widget::text_editor::Edit::Indent => {
                3.hash(state);
            }
            iced::widget::text_editor::Edit::Insert(ch) => {
                4.hash(state);
                ch.hash(state);
            }
            iced::widget::text_editor::Edit::Unindent => {
                5.hash(state);
            }
            iced::widget::text_editor::Edit::Paste(text) => {
                6.hash(state);
                text.hash(state);
            }
        }
    }
}

/// A wrapper for the XmlElement that implements Hash, so it can be used in iced's Message enum, which requires all types to implement Hash.
///
/// Usage: HashableXmlElement::new(...)
///
/// To get the value: HashableXmlElement.value()
#[derive(Debug, Clone)]
pub struct HashableXmlElement(XmlElement);

impl HashableXmlElement {
    /// Create a new HashableXmlElement from an XmlElement value
    ///
    /// Parameters:
    ///
    /// - value: the XmlElement value to wrap
    ///
    /// Returns:
    ///
    /// - a new HashableXmlElement
    pub fn new(value: XmlElement) -> Self {
        HashableXmlElement(value)
    }

    /// Get the XmlElement value from the HashableXmlElement
    ///
    /// No parameters.
    ///
    /// Returns:
    ///
    /// - the XmlElement value
    pub fn value(&self) -> &XmlElement {
        return &self.0;
    }
}

impl std::hash::Hash for HashableXmlElement {
    /// Hash the HashableXmlElement by hashing the underlying XmlElement
    ///
    /// Parameters:
    ///
    /// - state: the hasher to use
    ///
    /// Returns:
    ///
    /// - void
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the underlying XmlElement by hashing its tag, attributes, text, children, id, classes, and datas
        // !! Here we ignore the Style, since it would be too complex to hash, like literally hours of work
        self.0.tag.hash(state);
        HashableHashMap::new(self.0.attributes.clone()).hash(state);
        self.0.text.hash(state);
        for child in &self.0.children {
            HashableXmlElement::new(child.clone()).hash(state);
        }
        self.0.id.hash(state);
        self.0.classes.hash(state);
        HashableHashMap::new(self.0.datas.clone()).hash(state);
    }
}

///  A struct that represents the scroll state of an element, with scroll_left, scroll_top, scroll_width, and scroll_height as HashableF32 values.
///
///  Used by the <Scroll /> element
#[derive(Debug, Clone, Hash)]
pub struct ScrollState {
    pub scroll_left: HashableF32,
    pub scroll_top: HashableF32,
    pub scroll_width: HashableF32,
    pub scroll_height: HashableF32,
}

///  A basic Vector2 with two HashableF32 values, x and y. Used for positions, sizes, etc.
#[derive(Debug, Clone, Hash)]
pub struct Vector2 {
    pub x: HashableF32,
    pub y: HashableF32,
}

///  Checks if an &str is "alphabetic"-only, but also allows '-' and '_'
///
///  Parameters:
///
///  - s: the &str to check
///
///  Returns:
///
///  - true if the &str is "alphabetic"-only, false otherwise
pub fn is_alphabetic(s: &str) -> bool {
    return s.chars().all(|c| c.is_alphabetic() || c == '-' || c == '_');
}

///  Converts degrees to radians
///
///  Parameters:
///
///  - degrees: the degrees to convert
///
///  Returns:
///
///  - the radians
pub fn to_rad(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
