mod error;

use self::error::{SnlMarkupError, SnlMarkupResult};

fn search_pattern(s: &str, start: usize, patt: &str) -> Option<usize> {
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
fn search_pattern_rec<R>(
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

struct SnlMarkup {
    markup_content: String,
    pattern_command_ident_and_lcurl_offset: (usize, usize),
}

const PATTERN_COMMAND: &str = "\\pattern";
const PATTERN_ARG_COMMAND: &str = "\\patternArg";
const LCURL: &str = "{";
const RCURL: &str = "}";

impl TryFrom<&str> for SnlMarkup {
    type Error = SnlMarkupError;

    fn try_from(s: &str) -> SnlMarkupResult<Self> {
        // Find start of pattern block
        let pattern_command_ident_and_lcurl_offset =
            search_pattern_rec(s, 0, PATTERN_COMMAND, |s, n| {
                let after_pattern = &s[(n + PATTERN_COMMAND.len())..];
                let trimmed = after_pattern.trim_start();
                trimmed
                    .starts_with(LCURL)
                    .then(|| n + PATTERN_COMMAND.len() + after_pattern.len() - trimmed.len() + 1)
            })
            .ok_or(SnlMarkupError::CoundntFindPatternCommand {
                markup_content: s.to_string(),
            })?;

        Ok(Self::new(
            s.to_string(),
            pattern_command_ident_and_lcurl_offset,
        ))
    }
}

impl SnlMarkup {
    fn new(markup_content: String, pattern_command_ident_and_lcurl_offset: (usize, usize)) -> Self {
        let slf = Self {
            markup_content,
            pattern_command_ident_and_lcurl_offset,
        };
        slf.check_markup_validity();
        slf
    }

    /// this is for sanity checking
    fn check_markup_validity(&self) {
        let (start, end) = self.pattern_command_ident_and_lcurl_offset;
        assert!(
            start < self.markup_content.len(),
            "pattern command start position {} exceeds content length {}",
            start,
            self.markup_content.len()
        );
        assert!(
            end <= self.markup_content.len(),
            "left curly brace position {} exceeds content length {}",
            end,
            self.markup_content.len()
        );
        assert!(
            self.markup_content[start..end].starts_with(PATTERN_COMMAND),
            "expected pattern command '{}' at position {}, found: '{}'",
            PATTERN_COMMAND,
            start,
            &self.markup_content[start..end]
        );
        assert!(
            self.markup_content[(end - 1)..end].ends_with(LCURL),
            "expected '{}' at position {}, found: '{}'",
            LCURL,
            end - 1,
            &self.markup_content[(end - 1)..end]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn snl_markup_try_from_str_works() {
        fn t(input: &str, expect: expect_test::Expect) {
            let markup = SnlMarkup::try_from(input).expect("Should parse valid markup");
            expect.assert_eq(&format!(
                "{:?}",
                (
                    markup.markup_content,
                    markup.pattern_command_ident_and_lcurl_offset,
                )
            ));
        }

        t(
            r#"\pattern { hello }"#,
            expect![[r#"("\\pattern { hello }", (0, 10))"#]],
        );

        t(
            r#"\pattern    { test }"#,
            expect![[r#"("\\pattern    { test }", (0, 13))"#]],
        );
    }
}
