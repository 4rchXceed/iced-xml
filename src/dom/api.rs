use crate::{
    css_reader::SelectorType,
    dom::{events::DomQueryBuilder, query::DomQuery},
};

/// Dom API
///
/// Used with a window
/// Examples:
/// self.qb.q(Dom::get_element_by_id("test-btn")).with_callback(|el| ...);
pub struct Dom {}

impl Dom {
    /// Selects by element UID.
    ///
    /// UID => internal element ID.
    ///
    /// Using it is a bad idea, use IDs or classes instead.
    pub fn get_element<'a>(uid: i32) -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::Uid, uid.to_string());
    }

    /// Selects by element ID.
    ///
    /// CSS equivalent: #id
    ///
    /// Always point to a SINGLE element.
    pub fn get_element_by_id<'a>(id: &str) -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::Id, id.to_string());
    }

    /// Selects by element class.
    ///
    /// CSS equivalent: .class
    ///
    /// Can point to MULTIPLE elements.
    pub fn get_elements_by_class<'a>(class: &str) -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::Class, class.to_string());
    }

    /// Selects by element tag.
    ///
    /// CSS equivalent: Tag
    ///
    /// Can point to MULTIPLE elements.
    pub fn get_elements_by_tag<'a>(tag: &str) -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::Tag, tag.to_string());
    }

    /// Selects by complex CSS query.
    ///
    /// CSS equivalent: query ~ subquery subquery > ...
    ///
    /// "," NOT SUPPORTED!
    pub fn query_selector<'a>(query: &str) -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::Complex, query.to_string());
    }

    /// Selects all elements.
    ///
    /// CSS equivalent: *
    ///
    /// Can point to MULTIPLE elements.
    pub fn all() -> DomQueryBuilder {
        return DomQueryBuilder::new(SelectorType::All, "".to_string());
    }

    /// Converts a DomQuery into a DomQueryResult.
    ///
    /// DomQueries can be get from event_response.target
    ///
    /// No CSS equivalent, this is a special case.
    ///
    /// Always point to a SINGLE element.
    pub fn from(query: DomQuery) -> DomQueryBuilder {
        return DomQueryBuilder::from(query);
    }
}
