mod error;
mod search;

use self::{
    error::{SnlMarkupError, SnlMarkupResult},
    search::search_pattern_rec,
};
use eterned::db::EternerDb;
use husky_text_protocol::offset::TextOffsetRange;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use snl_prelude::coword::{SnlIdent, SnlIdentMap};

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnlMarkup {
    markup_content: String,
    pattern_command_ident_and_lcurl_offset_range: TextOffsetRange,
}

const PATTERN_COMMAND: &str = "\\pattern";
const PATTERN_ARG_COMMAND: &str = "\\patternArg";
const LCURL: &str = "{";
const RCURL: &str = "}";

impl TryFrom<String> for SnlMarkup {
    type Error = SnlMarkupError;

    fn try_from(markup_content: String) -> SnlMarkupResult<Self> {
        // Find start of pattern block
        let Some(pattern_command_ident_and_lcurl_offset_range) =
            search_pattern_rec(&markup_content, 0, PATTERN_COMMAND, |s, n| {
                let after_pattern = &s[(n + PATTERN_COMMAND.len())..];
                let trimmed = after_pattern.trim_start();
                trimmed
                    .starts_with(LCURL)
                    .then(|| n + PATTERN_COMMAND.len() + after_pattern.len() - trimmed.len() + 1)
            })
        else {
            return Err(SnlMarkupError::CoundntFindPatternCommand { markup_content });
        };
        Ok(Self::new(
            markup_content,
            pattern_command_ident_and_lcurl_offset_range.into(),
        ))
    }
}

pub struct PatternArgumentValue {
    content: String,
}

pub type PatternArgumentValues = SmallVec<[PatternArgumentValue; 4]>;

pub struct SnlMarkupPatternArgument {
    pattern_argument_command_ident_and_lcurl_offset_range: TextOffsetRange,
    pattern_argument_key_ident_offset_range: TextOffsetRange,
    pattern_argument_value_lcurl_offset_range: TextOffsetRange,
}

pub type SnlMarkupPatternArguments = SmallVec<[SnlMarkupPatternArgument; 4]>;

impl SnlMarkup {
    fn new(
        markup_content: String,
        pattern_command_ident_and_lcurl_offset_range: TextOffsetRange,
    ) -> Self {
        let slf = Self {
            markup_content,
            pattern_command_ident_and_lcurl_offset_range,
        };
        slf.check_markup_validity();
        slf
    }

    fn calc_pattern_argument_values(
        markup_content: &str,
        pattern_arguments: &[SnlMarkupPatternArgument],
        db: &EternerDb,
    ) -> SnlMarkupResult<SnlIdentMap<PatternArgumentValues>> {
        let mut pattern_argument_values_map: SnlIdentMap<PatternArgumentValues> =
            SnlIdentMap::default();
        for pattern_argument in pattern_arguments {
            let pattern_argument_key = SnlIdent::from_ref(
                &markup_content[pattern_argument.pattern_argument_key_ident_offset_range],
                db,
            )
            .map_err(|e| todo!())?;
            let pattern_argument_value =
                &markup_content[pattern_argument.pattern_argument_value_lcurl_offset_range];
            pattern_argument_values_map.update_value_or_insert_with(
                pattern_argument_key,
                |values| {
                    values.push(PatternArgumentValue {
                        content: pattern_argument_value.to_string(),
                    })
                },
                || {
                    smallvec![PatternArgumentValue {
                        content: pattern_argument_value.to_string(),
                    }]
                },
            );
        }
        Ok(pattern_argument_values_map)
    }

    /// this is for sanity checking
    fn check_markup_validity(&self) {
        let range @ TextOffsetRange { start, end } =
            self.pattern_command_ident_and_lcurl_offset_range;
        assert!(
            start.index() < self.markup_content.len(),
            "pattern command start position {} exceeds content length {}",
            start,
            self.markup_content.len()
        );
        assert!(
            end.index() <= self.markup_content.len(),
            "left curly brace position {} exceeds content length {}",
            end,
            self.markup_content.len()
        );
        assert!(
            self.markup_content[range].starts_with(PATTERN_COMMAND),
            "expected pattern command '{}' at position {}, found: '{}'",
            PATTERN_COMMAND,
            start,
            &self.markup_content[range]
        );
        assert!(
            self.markup_content[(end.index() - 1)..end.index()].ends_with(LCURL),
            "expected '{}' at position {}, found: '{}'",
            LCURL,
            end.index() - 1,
            &self.markup_content[(end.index() - 1)..end.index()]
        );
    }
}

impl Serialize for SnlMarkup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.markup_content)
    }
}

impl<'de> Deserialize<'de> for SnlMarkup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let markup = SnlMarkup::try_from(s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(markup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn snl_markup_try_from_str_works() {
        fn t(input: &str, expect: expect_test::Expect) {
            let markup = SnlMarkup::try_from(input.to_string()).expect("Should parse valid markup");
            expect.assert_debug_eq(&markup);
        }

        t(
            r#"\pattern { hello }"#,
            expect![[r#"
                SnlMarkup {
                    markup_content: "\\pattern { hello }",
                    pattern_command_ident_and_lcurl_offset_range: 0..10,
                }
            "#]],
        );

        t(
            r#"\pattern    { test }"#,
            expect![[r#"
                SnlMarkup {
                    markup_content: "\\pattern    { test }",
                    pattern_command_ident_and_lcurl_offset_range: 0..13,
                }
            "#]],
        );
    }
}
