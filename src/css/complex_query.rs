use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComplexQueryError {
    #[error("Found select join keyword '{0}' but keyword '{1}' was already found, in query: {2}")]
    TwoKeywordsFound(char, char, String),
    #[error("Invalid complex selector, either empty or invalid: {0}")]
    EmptyComplexSelector(String),
}

/// This function splits a query into parts
/// A complex query is like this:
///
/// .selector1 > #selector2 + Tag1 ~ .selector3 => vec![(".selector1", None), ("#selector2", '>'), ("Tag1", '+'), (".selector3", '~')]
///
/// Parameters:
/// - query: the query to split
/// Returns:
/// - Vec<(String, Option<char>)>: the query splitted
pub fn split_complex_selector(
    query: String,
) -> Result<Vec<(String, Option<char>)>, ComplexQueryError> {
    let mut chars = query.chars();
    let mut next_op = chars.next();

    let mut all_parts = Vec::new();
    let mut current_part: String = String::new();
    let mut keyword: Option<char> = None;

    while let Some(next) = next_op {
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
            if let Some(operator) = operator {
                if let Some(keyword) = keyword
                    && keyword != ' '
                {
                    return Err(ComplexQueryError::TwoKeywordsFound(
                        operator, keyword, query,
                    ));
                }
                if !current_part.is_empty() {
                    all_parts.push((current_part.clone(), keyword));

                    current_part.clear();
                }
                keyword = Some(operator);
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
        next_op = chars.next();
    }

    if keyword.is_none() | current_part.is_empty() {
        return Err(ComplexQueryError::EmptyComplexSelector(query));
    }

    all_parts.push((current_part.clone(), keyword));

    return Ok(all_parts);
}
