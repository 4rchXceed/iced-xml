use std::panic;

#[derive(Clone)]
pub struct Selector {
    pub selector_type: String,
    pub content: String,
}

#[derive(Clone)]
pub struct RuleBlock {
    pub selectors: Vec<Selector>,
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub value: String,
}

impl Rule {
    pub fn hash_with_selector(&self, selector: &Selector) -> String {
        return format!(
            "{}:{}:{}",
            selector.selector_type, selector.content, self.name
        );
    }
}

#[derive(Clone)]
pub struct CssReader {
    input: String,
    pos: usize,
    pub rules: Vec<RuleBlock>,
}

impl CssReader {
    pub fn new(css: &str) -> Self {
        Self {
            input: css.to_string(),
            pos: 0,
            rules: Vec::new(),
        }
    }

    pub fn cur_unchecked(&self) -> char {
        let current = self.input[self.pos..].chars().next();
        if current.is_none() {
            panic!("Unexpected end of input at position {}", self.pos);
        } else {
            return current.unwrap();
        }
    }

    pub fn peek_unchecked(&self) -> char {
        let current = self.input[self.pos..].chars().next();
        if current.is_none() {
            panic!("Unexpected end of input at position {}", self.pos);
        }
        return current.unwrap();
    }

    pub fn skip_whitespace(&mut self) {
        while self.pos < self.input.len()
            && self.input[self.pos..]
                .starts_with(|c: char| c.is_whitespace() || c == '\n' || c == '\r')
        {
            self.pos += 1;
        }
    }

    pub fn parse(&mut self) {
        self.skip_whitespace();
        while self.pos < self.input.len() {
            self.skip_whitespace();
            let selectors = self.parse_selectors();
            let rules = self.parse_rules();
            self.rules.push(RuleBlock { selectors, rules });
            self.skip_whitespace();
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = Vec::new();
        while self.cur_unchecked() != '{' {
            self.skip_whitespace();
            selectors.push(self.parse_selector());
            self.skip_whitespace();
        }
        self.pos += 1; // Skip the opening '{'
        self.skip_whitespace();
        return selectors;
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = Vec::new();
        while self.cur_unchecked() != '}' {
            self.skip_whitespace();
            rules.push(self.parse_rule());
            self.skip_whitespace();
        }
        self.pos += 1; // Skip the closing '}'
        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        let mut name = String::new();
        let mut value = String::new();
        let mut reading_name = true;
        let mut finished = false;
        while self.pos < self.input.len() && !finished {
            let current = self.cur_unchecked();
            if current == ':' && reading_name {
                reading_name = false;
                self.skip_whitespace();
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
        return Rule {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        };
    }

    fn parse_selector(&mut self) -> Selector {
        self.skip_whitespace();
        let mut selector = String::new();
        let mut reading_selector = true;
        while self.pos < self.input.len() && reading_selector {
            let current = self.cur_unchecked();
            if current == ',' {
                self.pos += 1;
                reading_selector = false;
            } else if current == '{' {
                reading_selector = false;
            } else if !current.is_whitespace() {
                self.pos += 1;
                selector.push(current);
            } else {
                self.pos += 1;
            }
        }
        self.skip_whitespace();
        if !selector.is_empty() {
            if selector.starts_with("#") {
                return Selector {
                    selector_type: "id".to_string(),
                    content: selector.strip_prefix("#").unwrap().to_string(),
                };
            } else if selector.starts_with(".") {
                return Selector {
                    selector_type: "class".to_string(),
                    content: selector.strip_prefix(".").unwrap().to_string(),
                };
            } else if selector == "*" {
                return Selector {
                    selector_type: "all".to_string(),
                    content: selector,
                };
            } else {
                panic!("Tag selectors are not supported yet. Found: {}", selector);
                //TODO
                // return Selector {
                //     selector_type: "tag".to_string(),
                //     content: selector,
                // };
            }
        } else {
            panic!("Expected selector at position {}", self.pos);
        }
    }
}
