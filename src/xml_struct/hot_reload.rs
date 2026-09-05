use std::collections::HashMap;

use crate::{
    css_reader::{Rule, RuleBlock, Selector},
    dom::query::DomQuery,
    xml_struct::{
        element_renderer::{ElementRenderer, HotReloadState, extract_selector_style_flag},
        theming::gen_styles,
    },
};

/// Cleans up the styles for hot reload by reverting any removed rules to their previous state.
///
/// Parameters:
/// - element_renderer: A mutable reference to the ElementRenderer that manages the elements and their styles.
/// - new_rules: A reference to a vector of RuleBlock representing the new set of rules after hot reload.
///
/// Note that this used to by inside ElementRenderer, but I move it out because it was too big
/// This works by comparing the old rules with the new rules, and for any rule that has been removed, it reverts the style changes made by that rule to the elements that were affected by it.
pub(super) fn cleanup_for_hot_reload(
    element_renderer: &mut ElementRenderer,
    new_rules: &Vec<RuleBlock>,
) {
    let font_list = element_renderer.get_font_list().clone();
    let old_state = element_renderer
        .hot_reload_states
        .clone()
        .unwrap_or(HotReloadState {
            rules: Vec::new(),
            state: HashMap::new(),
        });
    let mut state_hashed: HashMap<String, (Selector, Rule)> = HashMap::new();
    for rule_block in new_rules.iter() {
        for selector in &rule_block.selectors {
            for rule in &rule_block.rules {
                state_hashed.insert(
                    rule.hash_with_selector(selector),
                    (selector.clone(), rule.clone()),
                );
            }
        }
    }
    // Ok, small explaination of what's going on here
    for old_rule_block in old_state.rules.iter() {
        for selector in &old_rule_block.selectors {
            for rule in &old_rule_block.rules {
                handle_single_rule(
                    element_renderer,
                    font_list.clone(),
                    &old_state,
                    &state_hashed,
                    selector,
                    rule,
                );
            }
        }
    }
}

/// Handles a single rule during the hot reload cleanup process.
///
/// This function checks if a rule has been removed, and calls the appropriate processing function to revert the style changes made by that rule.
/// I'm not going to comment the parameters because they are the same as cleanup_for_hot_reload, but for a single rule
fn handle_single_rule(
    element_renderer: &mut ElementRenderer,
    font_list: Vec<(String, &'static str)>,
    old_state: &HotReloadState,
    state_hashed: &HashMap<String, (Selector, Rule)>,
    selector: &Selector,
    rule: &Rule,
) {
    let hash = rule.hash_with_selector(selector);
    // Here: rule has been removed, we need to revert the style change
    if !state_hashed.contains_key(&hash) {
        let (elements, flag) = get_element(element_renderer, selector);
        // Then we loop through all the elements and revert the style change
        for element in elements {
            if flag.is_some() {
                process_flag_element(
                    element_renderer,
                    &font_list,
                    old_state,
                    rule,
                    &flag,
                    element,
                );
            } else {
                process_element(element_renderer, &font_list, old_state, rule, element); // Normally, we should always find the old theme, but if we don't, we just skip it
            }
        }
    }
}

/// Processes an element with a specific flag during the hot reload cleanup process.
///
/// First, the function will get the old theme, and the new theme.
/// Then it'll call the apply_only_changes function, which will revert the style changes made by the rule to the element with the specific flag.
/// See: apply_only_changes
fn process_flag_element(
    element_renderer: &mut ElementRenderer,
    font_list: &Vec<(String, &'static str)>,
    old_state: &HotReloadState,
    rule: &Rule,
    flag: &Option<String>,
    element: i32,
) {
    let old_themes = old_state.state.get(&element);
    if old_themes.is_some() {
        let mut old_flag_theme_op = old_themes
            .as_ref()
            .unwrap()
            .flag_themes
            .get(&flag.as_ref().unwrap().to_string())
            .cloned();
        if old_flag_theme_op.is_none() {
            old_flag_theme_op = Some(old_themes.as_ref().unwrap().default_theme.clone());
        }

        let mut old_flag_theme = old_flag_theme_op.unwrap();
        let real_element = element_renderer.elements.get_mut(&element);
        if real_element.is_some() {
            let rule_to_change = old_flag_theme.clone();
            gen_styles(&rule.name, &rule.value, &mut old_flag_theme, font_list);
            let (_, datas) = real_element.unwrap();
            let mut flag_theme = datas
                .flag_themes
                .get_mut(&flag.as_ref().unwrap().to_string());
            if flag_theme.is_some() {
                flag_theme.as_mut().unwrap().apply_only_changes(
                    &old_flag_theme,
                    &rule_to_change,
                    &rule_to_change.clone(),
                );
            }
        }
    }
}

/// Processes an element without a specific flag during the hot reload cleanup process.
///
/// First, the function will get the old theme, and the new theme.
/// Then it'll call the apply_only_changes function, which will revert the style changes made by the rule to the element without a specific flag.
/// See: apply_only_changes
fn process_element(
    element_renderer: &mut ElementRenderer,
    font_list: &Vec<(String, &'static str)>,
    old_state: &HotReloadState,
    rule: &Rule,
    element: i32,
) {
    // get the old theme, before the first css hot-reload-supported change (hot-reload-supported is when hot_reload is true)
    let old_theme = old_state.state.get(&element);
    // If we found the old theme, we can revert the style change
    if old_theme.is_some() {
        // Get the real element
        let real_element = element_renderer.elements.get_mut(&element);
        // Then we clone the old theme, remove the style change from a "virtual" theme
        let mut rule_to_change = old_theme.unwrap().default_theme.clone();
        gen_styles(&rule.name, &rule.value, &mut rule_to_change, font_list);
        if real_element.is_some() {
            let (_, datas) = real_element.unwrap();
            // And revert the style change by applying only the changes from the old theme to the new theme
            datas.default_theme.apply_only_changes(
                &old_theme.unwrap().default_theme.clone(),
                &rule_to_change,
                &old_theme.unwrap().default_theme.clone(),
            );
            // By doing all of the gen_styles and other stuff, we avoid having the create a revert function (from XmlTheme to rules)
            // We still need apply_only_changes tho
        }
    }
}

/// Gets the elements that match the selector, and returns them as a vector of i32 (the element ids), and the flag if it exists
fn get_element(
    element_renderer: &mut ElementRenderer,
    selector: &Selector,
) -> (Vec<i32>, Option<String>) {
    // We are selecting the correct element, so we can work on it
    let query = DomQuery::new(
        selector.selector_type.clone(),
        selector.content.clone(),
        selector.flag.clone(),
    );
    let elements: Vec<i32> = element_renderer
        .element_query(&query)
        .iter()
        .map(|e| element_renderer.post_process_query_result(&query, e.clone()))
        .flatten()
        .collect();
    let mut flag: Option<String> = None;
    if selector.flag.is_some() {
        flag = extract_selector_style_flag(&selector.flag.as_ref().unwrap());
    }
    return (elements, flag);
}
