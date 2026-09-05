use crate::css_reader::{CssReader, Selector, SelectorType, split_complex_selector};

/// Complex query join types for DOM queries (refer to the source code's comments for more details)
#[derive(Debug)]
pub enum ComplexQueryJoinType {
    Descendant, // " "
    Child,      // ">"
    Silbling,   // "~"
    Also,       // Tag#id.class
}

impl ComplexQueryJoinType {
    /// Creates a new Self from a character.
    /// " " => Descendant
    /// ">" => Child
    /// "~" => Silbling
    /// "=" => Also
    /// None => None
    /// Other => None
    pub fn from(s: Option<char>) -> Option<Self> {
        if s.is_none() {
            return None;
        } else {
            return match s.as_ref().unwrap() {
                ' ' => Some(ComplexQueryJoinType::Descendant),
                '>' => Some(ComplexQueryJoinType::Child),
                '~' => Some(ComplexQueryJoinType::Silbling),
                '=' => Some(ComplexQueryJoinType::Also),
                _ => None,
            };
        }
    }
}

/// Represents a complex query.
#[derive(Debug)]
pub struct ComplexQuery {
    pub query: DomQueryType,
    pub next: Option<Box<ComplexQuery>>,
    pub link_next: Option<ComplexQueryJoinType>,
}

impl ComplexQuery {
    /// Creates a new ComplexQuery from a query part and an optional link character.
    ///
    /// Parameters:
    /// - query_part: The query part as a string.
    /// - link: An optional character representing the link type.
    ///
    /// Returns:
    /// A new instance of ComplexQuery.
    pub fn new(mut query_part: String, link: Option<char>) -> Self {
        query_part.push_str(",");
        let query: Selector = CssReader::new(query_part.as_str()).parse_selector();
        let query_type = gen_query_type(query.selector_type, query.content);
        match query_type {
            DomQueryType::Complex(_) => {
                println!(
                    "Complex query type is not supported in ComplexQuery: {:?}",
                    query_type
                );
                return Self {
                    query: DomQueryType::Unused,
                    next: None,
                    link_next: ComplexQueryJoinType::from(link),
                };
            }
            _ => {}
        }
        return Self {
            query: query_type,
            next: None,
            link_next: ComplexQueryJoinType::from(link),
        };
    }

    /// Continues parsing the complex query from a vector of query parts.
    ///
    /// Parameters:
    /// - full: A vector of tuples containing query parts and optional link characters.
    ///
    /// Returns:
    /// A new instance of ComplexQuery representing the next part of the complex query.
    fn next(mut full: Vec<(String, Option<char>)>) -> Self {
        let (query_part, link_next) = full.remove(0);
        let mut base = ComplexQuery::new(query_part, link_next);
        if full.len() > 0 {
            base.next = Some(Box::new(ComplexQuery::next(full)));
        }
        return base;
    }

    /// Parses a full complex query string into a ComplexQuery structure.
    ///
    /// Parameters:
    /// - full_query: The full complex query string.
    ///
    /// Returns:
    /// A new instance of ComplexQuery representing the parsed complex query.
    pub fn from(full_query: String) -> Self {
        let full = split_complex_selector(full_query);
        return ComplexQuery::next(full);
    }
}

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
        SelectorType::Uid => DomQueryType::ByUid(val.parse::<i32>().unwrap()),
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
