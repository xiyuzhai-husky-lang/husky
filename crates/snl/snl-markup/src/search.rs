pub(crate) fn search_pattern(s: &str, start: usize, patt: &str) -> Option<usize> {
    if start >= s.len() || patt.is_empty() {
        return None;
    }

    s[start..].find(patt).map(|pos| pos + start)
}

/// Search and map pattern matches in a string to an optional value.
///
/// Iteratively:
/// 1. Finds the next occurrence of `patt` in the string starting from `start`
/// 2. Attempts to map the match to an optional value using the provided `map` function
/// 3. If mapping produces Some value, returns the position and that value
/// 4. If mapping produces None, continues searching from after the current match
///
/// Returns `Some((position, mapped_value))` for the first successful match and mapping,
/// or `None` if no successful mapping is found.
pub(crate) fn search_pattern_rec<R>(
    s: &str,
    mut start: usize,
    patt: &str,
    map: impl Fn(&str, usize) -> Option<R>,
) -> Option<(usize, R)> {
    while let Some(pos) = search_pattern(s, start, patt) {
        if let Some(result) = map(s, pos) {
            return Some((pos, result));
        }
        start = pos + patt.len();
    }
    None
}

pub(crate) fn search_for_outstanding_rcurl(s: &str, start: usize) -> Option<usize> {
    let mut brace_count = 0;
    let mut chars = s[start..].char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '\\' && chars.peek().is_some() {
            // Skip the escaped character
            chars.next();
            continue;
        }

        match c {
            '{' => brace_count += 1,
            '}' => {
                if brace_count == 0 {
                    return Some(start + i);
                }
                brace_count -= 1;
            }
            _ => {}
        }
    }

    None
}

#[test]
fn search_for_outstanding_rcurl_works() {
    #[track_caller]
    fn t(input: &str, expected_after_rcurl: &str) {
        match search_for_outstanding_rcurl(input, 0) {
            Some(pos) => {
                assert_eq!(
                    &input[pos..],
                    expected_after_rcurl,
                    "Input: '{}', Expected: '{}', Got: '{}'",
                    input,
                    expected_after_rcurl,
                    &input[pos..]
                );
            }
            None => {
                assert!(
                    expected_after_rcurl.is_empty(),
                    "Expected None with input matching expected"
                );
            }
        }
    }

    // No outstanding right curly brace
    t(r#"a{b{c}d}e"#, "");
    t(r#"\{"#, "");
    t(r#"\}"#, "");
    t(r#"\{\}"#, "");
    t(r#"\text\{abc\}"#, "");
    t(r#"\{\text{abc}\}"#, "");
    t(r#"\frac\{1\}\{2\}"#, "");
    t(r#"\{\{\{\}"#, "");

    // Outstanding right curly brace
    t(r#"a\{b}c"#, "}c");
    t(r#"a{b\}c}d"#, "");
    t(r#"\{b{c}d}e"#, "}e");
    t(r#"a{b\{c}d}e"#, "}e");
    t(r#"a{b{c\}}d}e"#, "");
    t(r#"a{b{c\}}d}}e"#, "}e");
    t(r#"\frac{1\}}{2}"#, "");
    t(r#"a\{b{c\}d}e}f"#, "}f");
    t(r#"\{a\}{b}c}"#, "}");
    t(r#"\\{a}\{b}c}"#, "}c}");
}
