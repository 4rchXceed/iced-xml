use crate::rs_utils::is_alphabetic;

// Utils
// Vec<String> -> [("#container", None), (".button", '>'), ("div", '+')]
// (String, Option<String>) -> String = selector, Option<String> = operator
pub fn split_complex_selector(query: String) -> Vec<(String, Option<char>)> {
    let mut query = query.chars();
    let mut next_op = query.next();
    let mut all_parts = Vec::new();
    let mut current_part: String = String::new();
    let mut keyword: Option<char> = None;
    while next_op.is_some() {
        let next = next_op.unwrap();
        if next.is_whitespace() {
            if !current_part.is_empty() {
                all_parts.push((current_part.clone(), keyword));
                current_part.clear();
                keyword = Some(' ');
            }
        } else {
            let operator = match next {
                '>' | '+' | '~' => Some(next),
                _ => None,
            };
            if operator.is_some() {
                if keyword.is_some() && keyword.unwrap() != ' ' {
                    println!("Wrong Complex selector: {:?}", query);
                    return Vec::new();
                }
                if !current_part.is_empty() {
                    all_parts.push((current_part.clone(), keyword));
                    current_part.clear();
                }
                keyword = operator;
            } else {
                let is_selector_char = match next {
                    '#' | '.' => true,
                    _ => false,
                };
                if is_selector_char && !current_part.is_empty() {
                    all_parts.push((current_part.clone(), keyword));
                    current_part.clear();
                    keyword = Some('=');
                }
                current_part.push(next);
            }
        }
        next_op = query.next();
    }
    if keyword.is_none() | current_part.is_empty() {
        println!("Wrong Complex selector: {:?}", query);
    }
    all_parts.push((current_part.clone(), keyword));

    return all_parts;
}

#[derive(Clone)]
pub struct Selector {
    pub selector_type: String,
    pub content: String,
    pub flag: Option<String>, // Supported: virtuals (parsed as: ::virtual)
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
            "{}:{}:{}:{}",
            selector.selector_type,
            selector.content,
            self.name,
            selector.flag.clone().unwrap_or("none".to_string())
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

    pub fn eof(&mut self) -> bool {
        self.pos >= self.input.chars().count()
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
        while !self.eof()
            && it < self.max_it
            && (self.cur_unchecked().is_whitespace()
                || self.cur_unchecked() == '\n'
                || self.cur_unchecked() == '\r'
                || self.cur_unchecked() == '/')
        {
            it += 1;
            // CSS Comments
            if self.cur_unchecked() == '/' && self.peek_unchecked() == '*' {
                self.pos += 2;
                let mut it = 0;
                while !self.eof()
                    && it < self.max_it
                    && !(self.cur_unchecked() == '*' && self.peek_unchecked() == '/')
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
        while !self.eof() && it < self.max_it {
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
        while !self.eof() && !finished && it < self.max_it {
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

    // Fn used by complex_selector
    pub fn parse_selector(&mut self) -> Selector {
        if self.kill_switch {
            return Selector {
                selector_type: String::from("none"),
                content: String::new(),
                flag: None,
            };
        }

        self.skip_whitespace();
        let mut selector = String::new();
        let mut reading_selector = true;
        let mut it = 0;
        while !self.eof() && reading_selector && it < self.max_it {
            it += 1;
            let current = self.cur_unchecked();
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
        self.skip_whitespace();
        let flag = selector
            .split("::")
            .collect::<Vec<&str>>()
            .get(1)
            .map(|s| s.to_string());
        let selector = selector.split("::").collect::<Vec<&str>>()[0]
            .trim()
            .to_string();
        if selector.is_empty() {
            self.kill_switch = true;
            self.kill_message = "Invalid selector at position ".to_string() + &self.pos.to_string();
            return Selector {
                selector_type: String::from("none"),
                content: String::new(),
                flag: None,
            };
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
            return Selector {
                selector_type: String::from("complex"),
                content: selector,
                flag: flag,
            };
        } else if selector.starts_with("#") {
            return Selector {
                selector_type: String::from("id"),
                content: selector.strip_prefix("#").unwrap().to_string(),
                flag: flag,
            };
        } else if selector.starts_with(".") {
            return Selector {
                selector_type: String::from("class"),
                content: selector.strip_prefix(".").unwrap().to_string(),
                flag: flag,
            };
        } else if selector == "*" {
            return Selector {
                selector_type: String::from("all"),
                content: selector,
                flag: flag,
            };
        } else if is_alphabetic(&selector) {
            return Selector {
                selector_type: String::from("tag"),
                content: selector,
                flag: flag,
            };
        } else {
            self.kill_switch = true;
            self.kill_message = "Invalid selector at position ".to_string() + &self.pos.to_string();
            return Selector {
                selector_type: String::from("none"),
                content: String::new(),
                flag: None,
            };
        }
    }
}
