mod error;
mod parser;
mod search;

use self::{
    error::{SnlMarkupError, SnlMarkupResult},
    search::search_pattern_rec,
};
use eterned::{attach::with_attached_eterner_db, db::EternerDb};
use husky_text_protocol::offset::{TextOffset, TextOffsetRange};
use parser::SnlMarkupParser;
use search::search_for_outstanding_rcurl;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use snl_prelude::coword::{SnlIdent, SnlIdentMap};

#[derive(PartialEq, Eq, Clone)]
struct SnlMarkup {
    markup_content: String,
    pattern_command_offset_range: TextOffsetRange,
    lcurl_offset: TextOffset,
    pattern_arguments: SnlMarkupPatternArguments,
    pattern_arguments_map: SnlIdentMap<SnlMarkupPatternArguments>,
    rcurl_offset: TextOffset,
}

impl std::fmt::Display for SnlMarkup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SnlMarkup")
            .field("markup_content", &self.markup_content)
            .field("pattern_arguments", &self.pattern_arguments)
            .finish()
    }
}

const PATTERN_COMMAND: &str = "π";
const PATTERN_ARGUMENT_COMMAND: &str = "α";
const LCURL: &str = "{";
const RCURL: &str = "}";

impl SnlMarkup {
    pub fn try_from_string(markup_content: String, db: &EternerDb) -> SnlMarkupResult<Self> {
        let mut parser = SnlMarkupParser::new(db, &markup_content);
        let Some(pattern_command_offset_range) = parser.parse_pattern_command() else {
            return Err(SnlMarkupError::CoundntFindPatternCommand { markup_content });
        };
        let Some((lcurl_offset, pattern_arguments, rcurl_offset)) =
            parser.parse_curly_braced(|parser| parser.parse_pattern_arguments())
        else {
            todo!()
        };
        let pattern_arguments = pattern_arguments?;
        let pattern_arguments_map =
            Self::calc_pattern_arguments_map(&markup_content, &pattern_arguments, db)?;
        let Some(rcurl_offset) = rcurl_offset else {
            todo!()
        };
        Ok(Self {
            markup_content,
            pattern_command_offset_range,
            lcurl_offset,
            pattern_arguments,
            pattern_arguments_map,
            rcurl_offset,
        })
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SnlMarkupPatternArgument {
    command_offset_range: TextOffsetRange,
    key_curled_offset_range: TextOffsetRange,
    key_ident: SnlIdent,
    value_curled_offset_range: TextOffsetRange,
    value_content: String,
}

impl std::fmt::Debug for SnlMarkupPatternArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{} = {}`", self.key_ident.data(), self.value_content)
    }
}

pub type SnlMarkupPatternArguments = SmallVec<[SnlMarkupPatternArgument; 4]>;

impl SnlMarkup {
    fn calc_pattern_arguments_map(
        markup_content: &str,
        pattern_arguments: &[SnlMarkupPatternArgument],
        db: &EternerDb,
    ) -> SnlMarkupResult<SnlIdentMap<SnlMarkupPatternArguments>> {
        let mut pattern_argument_values_map: SnlIdentMap<SnlMarkupPatternArguments> =
            SnlIdentMap::default();
        for pattern_argument in pattern_arguments {
            pattern_argument_values_map.update_value_or_insert_with(
                pattern_argument.key_ident,
                |values| values.push(pattern_argument.clone()),
                || smallvec![pattern_argument.clone()],
            );
        }
        Ok(pattern_argument_values_map)
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
        let markup = with_attached_eterner_db(|db| {
            SnlMarkup::try_from_string(s, db).map_err(|e| serde::de::Error::custom(e.to_string()))
        })
        .unwrap()?;
        Ok(markup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn snl_markup_try_from_str_works() {
        fn t(db: &EternerDb, input: &str, expect: expect_test::Expect) {
            let markup = SnlMarkup::try_from_string(input.to_string(), db)
                .expect("Should parse valid markup");
            expect.assert_eq(&format!("{:#}", markup));
        }

        let db = &EternerDb::default();
        t(
            db,
            r#"π { hello }"#,
            expect![[r#"
                SnlMarkup {
                    markup_content: "π { hello }",
                    pattern_arguments: [],
                }"#]],
        );
        t(
            db,
            r#"π    { test }"#,
            expect![[r#"
                SnlMarkup {
                    markup_content: "π    { test }",
                    pattern_arguments: [],
                }"#]],
        );
        t(
            db,
            r#"π{α{lopd}{1} + α{ropd}{2}}"#,
            expect![[r#"
                SnlMarkup {
                    markup_content: "π{α{lopd}{1} + α{ropd}{2}}",
                    pattern_arguments: [
                        `lopd = 1`,
                        `ropd = 2`,
                    ],
                }"#]],
        );
    }
}
