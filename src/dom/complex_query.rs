use crate::{
    css::{complex_query::split_complex_selector, errors::CssParseError, parser::CssReader},
    dom::query::{DomQueryType, gen_query_type},
};

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
        match s {
            Some(' ') => Some(ComplexQueryJoinType::Descendant),
            Some('>') => Some(ComplexQueryJoinType::Child),
            Some('~') => Some(ComplexQueryJoinType::Silbling),
            Some('=') => Some(ComplexQueryJoinType::Also),
            _ => None,
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
    pub fn new(mut query_part: String, link: Option<char>) -> Result<Self, CssParseError> {
        // "hack" to make the query valid for parsing
        query_part.push_str(",");

        let query = CssReader::new(query_part.as_str()).parse_selector()?;

        let query_type = gen_query_type(query.selector_type, query.content);

        match query_type {
            DomQueryType::Complex(_) => {
                return Err(CssParseError::ComplexQueryInComplexQuery(query_part));
            }
            _ => {}
        }

        return Ok(Self {
            query: query_type,
            next: None,
            link_next: ComplexQueryJoinType::from(link),
        });
    }

    /// Continues parsing the complex query from a vector of query parts.
    ///
    /// Parameters:
    /// - full: A vector of tuples containing query parts and optional link characters.
    ///
    /// Returns:
    /// A new instance of ComplexQuery representing the next part of the complex query.
    fn next(mut full: Vec<(String, Option<char>)>) -> Result<ComplexQuery, CssParseError> {
        let (query_part, link_next) = full.remove(0);

        let mut base = ComplexQuery::new(query_part, link_next)?;

        if full.len() > 0 {
            base.next = Some(Box::new(ComplexQuery::next(full)?));
        }

        return Ok(base);
    }

    /// Parses a full complex query string into a ComplexQuery structure.
    ///
    /// Parameters:
    /// - full_query: The full complex query string.
    ///
    /// Returns:
    /// A new instance of ComplexQuery representing the parsed complex query.
    pub fn from(full_query: String) -> Result<ComplexQuery, CssParseError> {
        let full = split_complex_selector(full_query.clone())
            .map_err(|e| CssParseError::ComplexQueryError(e, full_query))?;

        return Ok(ComplexQuery::next(full)?);
    }
}
