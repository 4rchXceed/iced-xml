/// This represents a Selector Type
#[derive(Clone, Debug)]
pub enum SelectorType {
    Class,
    Id,
    Tag,
    Complex,
    None,
    All,
    Uid,
}

/// This represents a selector
/// Types of selectors:
/// .class
/// #id
/// Tag
/// <selector> + > ~ or ' ' <selector2> ...
#[derive(Clone, Debug)]
pub struct Selector {
    /// The SelectorType
    pub selector_type: SelectorType,
    /// The string content
    pub content: String,
    /// The potential flag (::*)
    pub flag: Option<String>, // Supported: virtuals (parsed as: ::virtual)
}

/// This represents a list of rules (used to represent "selector, selector")
#[derive(Clone)]
pub struct RuleBlock {
    pub selectors: Vec<Selector>,
    pub rules: Vec<Rule>,
}

/// This represents a single rule (key: value;)
#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub value: String,
}

impl Rule {
    /// Used to hash a key to a hashmap, this is *NOT* a Hash trait
    /// Parameters:
    /// - selector: the selector to hash with self
    /// Returns:
    /// - String: the "hashed" string
    pub fn hash_with_selector(&self, selector: &Selector) -> String {
        return format!(
            "{}:{}:{}:{}",
            match selector.selector_type {
                SelectorType::Class => "class",
                SelectorType::Complex => "complex",
                SelectorType::Id => "id",
                SelectorType::None => "none",
                SelectorType::Tag => "tag",
                SelectorType::All => "all",
                SelectorType::Uid => "uid",
            },
            selector.content,
            self.name,
            selector.flag.clone().unwrap_or("none".to_string())
        );
    }
}
