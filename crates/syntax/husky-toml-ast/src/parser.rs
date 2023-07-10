use crate::*;

use husky_toml_token::{TomlSpecialToken, TomlToken, TomlTokenVariant};

use smallvec::SmallVec;

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
            TomlTokenVariant::Word(word) => self.parse_key_value(*word),
            TomlTokenVariant::StringLiteral {
                val: _,
                multiline: _,
            } => todo!("make key value"),
            TomlTokenVariant::Err(_) => TomlLineGroup::Err,
        }
    }

    fn parse_section_title(mut self) -> TomlLineGroup {
        let mut title: SmallVec<[Coword; 2]> = Default::default();
        let kind: TomlSectionKind = TomlSectionKind::Normal;
        let token = self.tokens.next().ok_or(TomlAstError::Expect)?;
        match token.variant() {
            TomlTokenVariant::Comment => todo!(),
            TomlTokenVariant::Special(_) => todo!(),
            TomlTokenVariant::Word(word) => title.push(*word),
            TomlTokenVariant::StringLiteral {
                val: _,
                multiline: _,
            } => todo!(),
            TomlTokenVariant::Err(_) => todo!(),
        }
        loop {
            let Some(token) = self.tokens.next() else {
                todo!()
            };
            match token.variant() {
                TomlTokenVariant::Comment => todo!(),
                TomlTokenVariant::Special(TomlSpecialToken::RightBox) => break,
                TomlTokenVariant::Special(_) => todo!(),
                TomlTokenVariant::Word(_) => todo!(),
                TomlTokenVariant::StringLiteral {
                    val: _,
                    multiline: _,
                } => todo!(),
                TomlTokenVariant::Err(_) => todo!(),
            }
        }
        TomlLineGroup::SectionTitle { title, kind }
    }

    fn parse_key_value(mut self, word: Coword) -> TomlLineGroup {
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
        Some(self.exprs.alloc_one(match self.tokens.next()?.variant() {
            TomlTokenVariant::Comment => todo!(),
            TomlTokenVariant::Special(_) => todo!(),
            TomlTokenVariant::Word(word) => match self.db.word_db().dt_coword(*word) {
                "true" => TomlExpr::Boolean(true),
                "false" => TomlExpr::Boolean(false),
                _ => todo!(),
            },
            TomlTokenVariant::StringLiteral { val, multiline: _ } => TomlExpr::String(val.clone()),
            TomlTokenVariant::Err(_) => todo!(),
        }))
    }
}
