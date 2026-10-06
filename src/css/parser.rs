use crate::{
    css::{
        errors::CssParseError,
        types::{Rule, RuleBlock, Selector, SelectorType},
    },
    rs_utils::is_alphabetic,
};

/// This struct is the main CSS Parsing code
/// It's state is made of a position in an input.
/// Usage:
/// ```rust,ignore
/// let mut reader = CssReader::new("css here");
/// reader.parse();
/// let rules = reader.get_rules();
/// ```
#[derive(Clone)]
pub struct CssReader {
    input: String,
    pos: usize,
    max_it: usize,
    rules: Vec<RuleBlock>,
}

impl CssReader {
    /// Creates a CssReader from a string
    pub fn new(css: &str) -> Self {
        Self {
            input: css.to_string(),
            pos: 0,
            max_it: 100000, // Safeguard against infinite loops
            rules: Vec::new(),
        }
    }

    /// Gets all the rules in the CssReader
    /// Returns: the rules
    pub fn get_rules(&self) -> &Vec<RuleBlock> {
        return &self.rules;
    }

    /// Gets the context
    pub fn get_context(&self) -> String {
        let chars = self.input.chars().collect::<Vec<char>>();

        let line = chars.iter().take(self.pos).filter(|&&c| c == '\n').count() + 1;
        let col = chars
            .iter()
            .take(self.pos)
            .rev()
            .take_while(|&&c| c != '\n')
            .count()
            + 1;

        let line_text = chars
            .iter()
            .take(self.pos)
            .skip_while(|&&c| c != '\n')
            .collect::<String>();

        return format!("--> {}\nLine: {}, Col: {}", line_text, line, col);
    }

    /// Gets the current character, and if it's the end of the String (self.input), kill the CssReader
    pub fn cur_unchecked(&mut self) -> Result<char, CssParseError> {
        let current = self.input.chars().take(self.pos + 1).skip(self.pos).next();

        if let Some(c) = current {
            return Ok(c);
        } else {
            return Err(CssParseError::UnexpectedEndOfInput(self.get_context()));
        }
    }

    /// Is the current character at the end of the string
    /// Returns: bool
    pub fn eof(&mut self) -> bool {
        self.pos >= self.input.chars().count()
    }

    /// Peek the next character, and kill the CssReader if it's EOF
    /// Returns: self.input at self.pos + 1
    pub fn peek_unchecked(&mut self) -> Result<char, CssParseError> {
        let mut chars = self.input.chars().take(self.pos + 2).skip(self.pos + 1);

        let current = chars.next();

        if let Some(c) = current {
            return Ok(c);
        } else {
            return Err(CssParseError::UnexpectedEndOfInput(self.get_context()));
        }
    }

    /// Skips all whitespace and comments, until the next "normal" char
    pub fn skip_whitespace(&mut self) -> Result<(), CssParseError> {
        let mut it = 0;
        while !self.eof()
            && it < self.max_it
            && (self.cur_unchecked()?.is_whitespace()
                || self.cur_unchecked()? == '\n'
                || self.cur_unchecked()? == '\r'
                || self.cur_unchecked()? == '/')
        {
            it += 1;

            // CSS Comments
            if self.cur_unchecked()? == '/' && self.peek_unchecked()? == '*' {
                self.pos += 2;

                let mut it = 0;

                while !self.eof()
                    && it < self.max_it
                    && !(self.cur_unchecked()? == '*' && self.peek_unchecked()? == '/')
                {
                    it += 1;

                    self.pos += 1;
                }
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }

        return Ok(());
    }

    /// Parses the CSS as a file
    pub fn parse(&mut self) -> Result<(), CssParseError> {
        self.skip_whitespace()?;

        let mut it = 0;
        while !self.eof() && it < self.max_it {
            it += 1;

            self.skip_whitespace()?;

            let selectors = self.parse_selectors()?;
            let rules = self.parse_rules()?;

            self.rules.push(RuleBlock { selectors, rules });

            self.skip_whitespace()?;
        }

        return Ok(());
    }

    /// Parses selectors separated by a comma ",", returns a list of Selector
    fn parse_selectors(&mut self) -> Result<Vec<Selector>, CssParseError> {
        let mut selectors: Vec<Selector> = Vec::new();
        let mut it = 0;

        while self.cur_unchecked()? != '{' && it < self.max_it {
            it += 1;

            self.skip_whitespace()?;

            let selector = self.parse_selector()?;
            selectors.push(selector);

            self.skip_whitespace()?;
        }
        self.pos += 1; // Skip the opening '{'

        self.skip_whitespace()?;

        return Ok(selectors);
    }

    /// Parses all the CSS rules, like:
    ///     {
    /// key: value;
    /// }
    /// Returns a Vec<Rule>
    fn parse_rules(&mut self) -> Result<Vec<Rule>, CssParseError> {
        let mut rules: Vec<Rule> = Vec::new();
        let mut it = 0;

        while self.cur_unchecked()? != '}' && it < self.max_it {
            it += 1;

            self.skip_whitespace()?;

            let rule = self.parse_rule()?;
            rules.push(rule);

            self.skip_whitespace()?;
        }

        self.pos += 1; // Skip the closing '}'

        return Ok(rules);
    }

    /// Parses a single rule. (key: value;)
    /// Returns: a Rule object
    fn parse_rule(&mut self) -> Result<Rule, CssParseError> {
        let mut name = String::new();
        let mut value = String::new();

        let mut reading_name = true;
        let mut finished = false;
        let mut it = 0;

        while !self.eof() && !finished && it < self.max_it {
            it += 1;
            let current = self.cur_unchecked()?;

            if current == ':' && reading_name {
                reading_name = false;

                self.skip_whitespace()?;

                self.pos += 1;
            } else if current == ';' {
                finished = true;

                self.pos += 1;
            } else if reading_name {
                name.push(current);

                self.pos += 1;
            } else {
                value.push(current);

                self.pos += 1;
            }
        }

        return Ok(Rule {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        });
    }

    // pub since it's used elsewere
    // Parses a selector, and returns a Selector object
    pub fn parse_selector(&mut self) -> Result<Selector, CssParseError> {
        let mut selector = String::new();

        let mut reading_selector = true;
        let mut it = 0;

        self.skip_whitespace()?;
        while !self.eof() && reading_selector && it < self.max_it {
            it += 1;

            let current = self.cur_unchecked()?;

            if current == ',' {
                self.pos += 1;

                reading_selector = false;
            } else if current == '{' {
                reading_selector = false;
            } else {
                self.pos += 1;

                selector.push(current);
            }
        }

        self.skip_whitespace()?;
        let flag = selector
            .split("::")
            .collect::<Vec<&str>>()
            .get(1)
            .map(|s| s.trim().to_string());

        let selector = selector.split("::").collect::<Vec<&str>>()[0]
            .trim()
            .to_string();

        if selector.is_empty() {
            return Err(CssParseError::InvalidSelector(self.get_context()));
        }
        let selector_without_first_char = selector.chars().skip(1).collect::<String>();

        let is_complex = selector.contains(' ')
            // For Tag#id.class type selectors
            || selector_without_first_char.contains('#')
            || selector_without_first_char.contains('.')
            // For > ~ combinators
            || selector.contains('>')
            || selector.contains('~');

        if is_complex {
            return Ok(Selector {
                selector_type: SelectorType::Complex,
                content: selector,
                flag: flag,
            });
        } else if selector.starts_with("#") {
            return Ok(Selector {
                selector_type: SelectorType::Id,
                content: selector.strip_prefix("#").unwrap().to_string(),
                flag: flag,
            });
        } else if selector.starts_with(".") {
            return Ok(Selector {
                selector_type: SelectorType::Class,
                content: selector.strip_prefix(".").unwrap().to_string(),
                flag: flag,
            });
        } else if selector == "*" {
            return Ok(Selector {
                selector_type: SelectorType::All,
                content: selector,
                flag: flag,
            });
        } else if is_alphabetic(&selector) {
            return Ok(Selector {
                selector_type: SelectorType::Tag,
                content: selector,
                flag: flag,
            });
        } else {
            return Err(CssParseError::InvalidSelector(self.get_context()));
        }
    }
}
