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
    pub max_it: usize,
    pub rules: Vec<RuleBlock>,
    pub kill_switch: bool,
    pub kill_message: String,
}

impl CssReader {
    pub fn new(css: &str) -> Self {
        Self {
            input: css.to_string(),
            pos: 0,
            max_it: 100000, // Safeguard against infinite loops
            rules: Vec::new(),
            kill_switch: false,
            kill_message: "".to_string(),
        }
    }

    pub fn cur_unchecked(&mut self) -> char {
        let current = self.input.chars().take(self.pos + 1).skip(self.pos).next();
        if current.is_none() {
            self.kill_switch = true;
            self.kill_message = "Unexpected end of input (cur_unchecked)".to_string();
            return '\0';
        } else {
            return current.unwrap();
        }
    }

    pub fn peek_unchecked(&mut self) -> char {
        let mut chars = self.input.chars().take(self.pos + 2).skip(self.pos + 1);
        let current = chars.next();
        if current.is_none() {
            self.kill_switch = true;
            self.kill_message = "Unexpected end of input (peek_unchecked)".to_string();
            return '\0';
        }
        return current.unwrap();
    }

    pub fn skip_whitespace(&mut self) {
        if self.kill_switch {
            return;
        }
        let mut it = 0;
        while self.pos < self.input.len()
            && it < self.max_it
            && self.input[self.pos..]
                .starts_with(|c: char| c.is_whitespace() || c == '\n' || c == '\r' || c == '/')
        {
            it += 1;
            // CSS Comments
            if self.cur_unchecked() == '/' && self.peek_unchecked() == '*' {
                self.pos += 2;
                let mut it = 0;
                while self.pos < self.input.len()
                    && it < self.max_it
                    && !(self.input[self.pos..].starts_with("*/"))
                {
                    it += 1;
                    self.pos += 1;
                }
                self.pos += 2;
                continue;
            }
            self.pos += 1;
        }
    }

    pub fn parse(&mut self) {
        self.skip_whitespace();
        let mut it = 0;
        while self.pos < self.input.len() && it < self.max_it {
            it += 1;
            self.skip_whitespace();
            let selectors = self.parse_selectors();
            let rules = self.parse_rules();
            self.rules.push(RuleBlock { selectors, rules });
            self.skip_whitespace();
            if self.kill_switch {
                return;
            }
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        if self.kill_switch {
            return Vec::new();
        }
        let mut selectors: Vec<Selector> = Vec::new();
        let mut it = 0;
        while self.cur_unchecked() != '{' && it < self.max_it {
            it += 1;
            self.skip_whitespace();
            selectors.push(self.parse_selector());
            self.skip_whitespace();
        }
        self.pos += 1; // Skip the opening '{'
        self.skip_whitespace();
        return selectors;
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        if self.kill_switch {
            return Vec::new();
        }
        let mut rules: Vec<Rule> = Vec::new();
        let mut it = 0;
        while self.cur_unchecked() != '}' && it < self.max_it {
            it += 1;
            self.skip_whitespace();
            rules.push(self.parse_rule());
            self.skip_whitespace();
        }
        self.pos += 1; // Skip the closing '}'
        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        if self.kill_switch {
            return Rule {
                name: String::new(),
                value: String::new(),
            };
        }
        let mut name = String::new();
        let mut value = String::new();
        let mut reading_name = true;
        let mut finished = false;
        let mut it = 0;
        while self.pos < self.input.len() && !finished && it < self.max_it {
            it += 1;
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
        if self.kill_switch {
            return Selector {
                selector_type: String::new(),
                content: String::new(),
            };
        }

        self.skip_whitespace();
        let mut selector = String::new();
        let mut reading_selector = true;
        let mut it = 0;
        while self.pos < self.input.len() && reading_selector && it < self.max_it {
            it += 1;
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
                return Selector {
                    selector_type: "tag".to_string(),
                    content: selector,
                };
            }
        } else {
            self.kill_switch = true;
            self.kill_message = "Invalid selector at position ".to_string() + &self.pos.to_string();
            return Selector {
                selector_type: String::new(),
                content: String::new(),
            };
        }
    }
}
