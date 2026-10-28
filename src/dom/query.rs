use crate::css::types::SelectorType;

/// Represents the type of a DOM query.
///
/// CSS Equivalents:
/// ById(String): #id
/// ByUid(i32): None, by internal UID
/// Class(String): .class
/// Tag(String): tag
/// Complex(String): tag#id.class...
/// All: *
/// Unused: None
#[derive(Debug, Clone, Hash)]
pub enum DomQueryType {
    ById(String),
    ByUid(i32),
    Class(String),
    Tag(String),
    Complex(String),
    All,
    Unused,
}

/// Represents a DOM query with its type and an optional flag.
/// DOM Query => A query that can be used to select elements in the DOM.
#[derive(Debug, Clone, Hash)]
pub struct DomQuery {
    pub query_type: DomQueryType,
    pub flag: Option<String>,
}

/// Utility function to generate a DomQueryType from a SelectorType and a value.
pub fn gen_query_type(selector_type: SelectorType, val: String) -> DomQueryType {
    return match selector_type {
        SelectorType::Id => DomQueryType::ById(val),
        SelectorType::Uid => DomQueryType::ByUid(val.parse::<i32>().unwrap_or_default()), // This should never happen
        SelectorType::Class => DomQueryType::Class(val),
        SelectorType::Tag => DomQueryType::Tag(val),
        SelectorType::All => DomQueryType::All,
        SelectorType::Complex => DomQueryType::Complex(val),
        SelectorType::None => DomQueryType::Unused,
    };
}

impl DomQuery {
    /// Creates a DomQuery
    pub fn new(selector_type: SelectorType, val: String, flag: Option<String>) -> Self {
        return Self {
            query_type: gen_query_type(selector_type, val),
            flag: flag,
        };
    }
}
