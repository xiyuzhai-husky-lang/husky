use crate::*;
use husky_print_utils::p;
use husky_toml_token::{TomlSpecialToken, TomlToken, TomlTokenVariant};
use husky_toml_token_stream::TomlTokenStream;
use salsa::DebugWithDb;

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
            TomlTokenVariant::Comment => TomlLineGroup::Comment,
            TomlTokenVariant::Special(special) => match special {
                TomlSpecialToken::LeftBox => self.parse_section_title(),
                _ => todo!("unexpected"),
            },
            TomlTokenVariant::Keylike(word) => self.parse_key_value(*word),
            TomlTokenVariant::StringLiteral { val, multiline } => todo!("make key value"),
            TomlTokenVariant::Err(_) => TomlLineGroup::Err,
        }
    }

    fn parse_section_title(mut self) -> TomlLineGroup {
        let mut title: Vec<Word> = vec![];
        let mut is_scattered: bool = false;
        let token = self.tokens.next().ok_or(TomlAstError::Expect)?;
        match token.variant() {
            TomlTokenVariant::Comment => todo!(),
            TomlTokenVariant::Special(_) => todo!(),
            TomlTokenVariant::Keylike(word) => title.push(*word),
            TomlTokenVariant::StringLiteral { val, multiline } => todo!(),
            TomlTokenVariant::Err(_) => todo!(),
        }
        loop {
            let Some(token) = self.tokens.next() else {todo!()};
            match token.variant() {
                TomlTokenVariant::Comment => todo!(),
                TomlTokenVariant::Special(TomlSpecialToken::RightBox) => break,
                TomlTokenVariant::Special(_) => todo!(),
                TomlTokenVariant::Keylike(_) => todo!(),
                TomlTokenVariant::StringLiteral { val, multiline } => todo!(),
                TomlTokenVariant::Err(_) => todo!(),
            }
        }
        TomlLineGroup::SectionTitle {
            title,
            is_scattered,
        }
    }

    fn parse_key_value(mut self, word: Word) -> TomlLineGroup {
        self.eat_special(TomlSpecialToken::Equals);
        let expr = self.parse_expr();
        TomlLineGroup::KeyValue(word, expr)
    }

    fn eat_special(&mut self, target: TomlSpecialToken) -> Result<(), TomlAstError> {
        let token = self.tokens.next().ok_or(TomlAstError::Expect)?;
        match token.variant() {
            TomlTokenVariant::Special(special) if *special == target => Ok(()),
            _ => Err(todo!()),
        }
    }

    fn parse_expr(mut self) -> Option<TomlExprIdx> {
        Some(match self.tokens.next()?.variant() {
            TomlTokenVariant::Comment => todo!(),
            TomlTokenVariant::Special(_) => todo!(),
            TomlTokenVariant::Keylike(word) => {
                p!(word.debug(self.db.word_db()));
                todo!()
            }
            TomlTokenVariant::StringLiteral { val, multiline } => {
                self.exprs.alloc_one(TomlExpr::String(val.clone()))
            }
            TomlTokenVariant::Err(_) => todo!(),
        })
    }
}
