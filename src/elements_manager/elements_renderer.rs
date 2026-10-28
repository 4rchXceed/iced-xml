use std::collections::HashMap;

use crate::css::parser::CssReader;
use crate::css::types::{Rule, RuleBlock, Selector};
use crate::dom::query::DomQuery;
use crate::dom::types::DomInternalMessageType;
use crate::xml_struct::theming::{Fonts, gen_styles_log};

use super::hot_reload::cleanup_for_hot_reload;
use super::mappings::ElementMappings;
use super::utils::{HotReloadState, HotReloadStateEntry, StyleChangeEvent};

pub struct ElementsRenderer {
    /// Stores the state of the renderer for hot-reloading CSS, if any
    pub(super) hot_reload_states: Option<HotReloadState>,
    /// Stores the fonts
    pub(super) fonts: Fonts,
}

impl ElementsRenderer {
    pub fn new(fonts: Fonts) -> Self {
        Self {
            hot_reload_states: None,
            fonts: fonts,
        }
    }

    /// Loads a CSS string into the renderer.
    /// Uses CssReader to parse the CSS and apply the rules to the elements.
    /// Handles hot-reloading by updating the state and reverting old styles if necessary.
    ///
    /// Parameters:
    /// - css: The CSS string to load.
    /// - hot_reload: A boolean indicating whether this is a hot-reload operation (use false by default)
    pub fn load_css(
        &mut self,
        css: &str,
        hot_reload: bool,
        mappings: &mut ElementMappings,
    ) -> Result<Vec<(i32, DomInternalMessageType, bool)>, String> {
        let mut reader = CssReader::new(css);

        reader
            .parse()
            .map_err(|e| e.to_string())
            .map_err(|e| e.to_string())?;

        if hot_reload {
            if self.hot_reload_states.is_some() {
                self.update_state(&reader.get_rules());

                cleanup_for_hot_reload(
                    mappings,
                    &mut self.hot_reload_states,
                    &reader.get_rules(),
                    &self.fonts,
                );
            } else {
                self.hot_reload_states = Some(self.generate_state(mappings, &reader.get_rules()));
            }
        }

        let mut events: Vec<(i32, DomInternalMessageType, bool)> = Vec::new();

        for rule_block in reader.get_rules() {
            let selectors = &rule_block.selectors;
            for selector in selectors {
                events.append(&mut self.apply_rules(
                    mappings,
                    selector,
                    &rule_block.rules,
                    hot_reload,
                ));
            }
        }

        return Ok(events);
    }

    /// Generates the current state of the renderer for hot-reloading CSS.
    /// This function creates a snapshot of the current styles of all elements, which can be used to revert styles when hot-reloading CSS.
    fn generate_state(
        &mut self,
        mappings: &mut ElementMappings,
        rules: &Vec<RuleBlock>,
    ) -> HotReloadState {
        let state: HashMap<i32, HotReloadStateEntry> = mappings
            .elements
            .iter()
            .map(|(id, (_, datas))| {
                (
                    *id,
                    HotReloadStateEntry {
                        default_theme: datas.default_theme.clone(),
                        flag_themes: datas.flag_themes.clone(),
                    },
                )
            })
            .collect();

        return HotReloadState {
            rules: rules.clone(),
            state: state,
        };
    }

    /// Updates the current state of the renderer for hot-reloading CSS.
    ///
    /// Parameters:
    /// - rules: the new rules to update the state with
    fn update_state(&mut self, rules: &Vec<RuleBlock>) {
        // Update only new rules, and keep the old state for the rest
        let mut state_hashed: HashMap<String, (Selector, Rule)> = HashMap::new();

        if let Some(hot_reload_states) = &mut self.hot_reload_states {
            for rule_block in hot_reload_states.rules.iter() {
                for selector in &rule_block.selectors {
                    for rule in &rule_block.rules {
                        state_hashed.insert(
                            rule.hash_with_selector(selector),
                            (selector.clone(), rule.clone()),
                        );
                    }
                }
            }

            for rule_block in rules.iter() {
                for selector in &rule_block.selectors {
                    for rule in &rule_block.rules {
                        let hash = rule.hash_with_selector(selector);
                        if !state_hashed.contains_key(&hash) {
                            // Push the new rule
                            hot_reload_states.rules.push(rule_block.clone());
                        }
                    }
                }
            }
        }
    }

    /// Applies the given CSS rules to the elements selected by the given selector.
    ///
    /// Parameters:
    /// - selector: The element selector to apply the rules to.
    /// - rules: The CSS rules to apply.
    /// - comes_from_hot_reload: A boolean indicating whether this is a hot-reload operation (use false by default)
    pub fn apply_rules(
        &self,
        mappings: &mut ElementMappings,
        selector: &Selector,
        rules: &Vec<Rule>,
        comes_from_hot_reload: bool,
    ) -> Vec<(i32, DomInternalMessageType, bool)> {
        let dom_query = DomQuery::new(
            selector.selector_type.clone(),
            selector.content.clone(),
            selector.flag.clone(),
        );

        let elements = mappings.element_query(&dom_query);

        let custom_style_for_flag = match selector.flag.as_ref() {
            Some(flag) => ElementsRenderer::extract_selector_style_flag(flag),
            None => None,
        };

        let mut events = Vec::new();

        for element in elements {
            for rule in rules.iter() {
                events.push((
                    element,
                    DomInternalMessageType::StyleChange(StyleChangeEvent {
                        key: rule.name.clone(),
                        value: rule.value.clone(),
                        custom_flag: custom_style_for_flag.clone(),
                    }),
                    comes_from_hot_reload,
                ));
            }
        }

        return events;
    }

    /// Registers a style change event for hot-reloading, updating the stored state of the element's styles.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the element where the style change occurred.
    /// - event: The StyleChangeEvent containing the key, value, and optional custom flag for the style change.
    pub fn register_hot_reload_change(&mut self, uid: i32, event: &StyleChangeEvent) {
        // If the state exists
        if let Some(hot_reload_state) = &mut self.hot_reload_states {
            // If the element exists in the state
            if let Some(old_theme_op) = hot_reload_state.state.get_mut(&uid) {
                // Handle the flag possibility
                if let Some(flag) = &event.custom_flag {
                    let theme_entry = old_theme_op
                        .flag_themes
                        .entry(flag.clone())
                        .or_insert(old_theme_op.default_theme.clone());

                    gen_styles_log(&event.key, &event.value, theme_entry, &self.fonts);
                } else {
                    gen_styles_log(
                        &event.key,
                        &event.value,
                        &mut old_theme_op.default_theme,
                        &self.fonts,
                    );
                }
            }
        }
    }

    /// Updates the style for a specific element.
    ///
    /// Parameters:
    /// - uid: The unique identifier of the element to update.
    /// - event: The StyleChangeEvent containing the key, value, and optional custom flag for the style change.
    /// - comes_from_hot_reload: A boolean indicating whether this update is coming from a hot-reload operation (use false by default).
    pub fn update_style_for(
        &mut self,
        uid: i32,
        event: StyleChangeEvent,
        comes_from_hot_reload: bool,
        mappings: &mut ElementMappings,
    ) {
        // Update the hot reload state if it exists
        if self.hot_reload_states.is_some() && !comes_from_hot_reload {
            self.register_hot_reload_change(uid, &event);
        }
        if let Some((_, datas)) = mappings.elements.get_mut(&uid) {
            let theme = if let Some(flag) = &event.custom_flag {
                datas
                    .flag_themes
                    .entry(flag.clone())
                    .or_insert(datas.default_theme.clone())
            } else {
                &mut datas.default_theme
            };

            gen_styles_log(&event.key, &event.value, theme, &self.fonts);
        }
    }

    /// Extracts the custom style flag from a selector flag string, if it exists.
    /// The custom style flag is expected to be in the format "for(custom_flag)".
    ///
    /// Parameters:
    /// - flag: A reference to the selector flag string.
    ///
    /// Returns:
    /// - An Option<String> if the flag is successfully extracted.
    pub fn extract_selector_style_flag(flag: &String) -> Option<String> {
        let mut custom_style_for_flag: Option<String> = None;

        let flag = flag;

        if let Some(stripped) = flag.strip_prefix("for(").and_then(|s| s.strip_suffix(")")) {
            custom_style_for_flag = Some(stripped.to_string());
        }

        custom_style_for_flag
    }
}
