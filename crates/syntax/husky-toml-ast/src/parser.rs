use crate::*;
use husky_toml_token::{TomlSpecialToken, TomlToken, TomlTokenVariant};
use husky_toml_token_stream::TomlTokenStream;

pub(crate) struct TomlAstParser<'a> {
    db: &'a dyn TomlAstDb,
    tokens: TomlTokenStream<'a>,
    exprs: &'a mut TomlExprArena,
}

impl<'a> TomlAstParser<'a> {
    pub(crate) fn new(
        db: &'a dyn TomlAstDb,
        tokens: &'a [TomlToken],
        exprs: &'a mut TomlExprArena,
    ) -> Self {
        Self {
            db,
            tokens: TomlTokenStream::new(tokens),
            exprs,
        }
    }

    pub(crate) fn parse_line_group(mut self) -> TomlLineGroup {
        let first_token = self.tokens.next().unwrap();
        match first_token.variant() {
            Ok(variant) => match variant {
                TomlTokenVariant::Comment => TomlLineGroup::Comment,
                TomlTokenVariant::Special(special) => match special {
                    TomlSpecialToken::LeftBox => todo!("make section"),
                    _ => todo!("unexpected"),
                },
                TomlTokenVariant::Keylike(_) => todo!("make key value"),
                TomlTokenVariant::StringLiteral { val, multiline } => todo!("make key value"),
            },
            Err(_) => TomlLineGroup::Err,
        }
    }
}
