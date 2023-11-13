use crate::*;

use husky_toml_token::{TomlSpecialToken, TomlToken, TomlTokenData};

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
        match first_token.data() {
            TomlTokenData::Comment => TomlLineGroup::Comment,
            TomlTokenData::Special(special) => match special {
                TomlSpecialToken::LeftBox => self.parse_section_title(),
                _ => todo!("unexpected"),
            },
            TomlTokenData::Word(word) => self.parse_key_value(*word),
            TomlTokenData::StringLiteral {
                val: _,
                multiline: _,
            } => todo!("make key value"),
            TomlTokenData::Err(_) => TomlLineGroup::Err,
        }
    }

    fn parse_section_title(mut self) -> TomlLineGroup {
        let mut title: SmallVec<[Coword; 2]> = Default::default();
        let kind: TomlSectionKind = TomlSectionKind::Normal;
        let token = self.tokens.next().ok_or(TomlAstError::Expect)?;
        match token.data() {
            TomlTokenData::Comment => todo!(),
            TomlTokenData::Special(_) => todo!(),
            TomlTokenData::Word(word) => title.push(*word),
            TomlTokenData::StringLiteral {
                val: _,
                multiline: _,
            } => todo!(),
            TomlTokenData::Err(_) => todo!(),
        }
        loop {
            let Some(token) = self.tokens.next() else {
                todo!()
            };
            match token.data() {
                TomlTokenData::Comment => todo!(),
                TomlTokenData::Special(TomlSpecialToken::RightBox) => break,
                TomlTokenData::Special(_) => todo!(),
                TomlTokenData::Word(_) => todo!(),
                TomlTokenData::StringLiteral {
                    val: _,
                    multiline: _,
                } => todo!(),
                TomlTokenData::Err(_) => todo!(),
            }
        }
        TomlLineGroup::SectionTitle { title, kind }
    }

    fn parse_key_value(mut self, word: Coword) -> TomlLineGroup {
        match self.eat_special(TomlSpecialToken::Equals) {
            Ok(_) => (),
            Err(_) => return TomlLineGroup::Err,
        };
        let expr = self.parse_expr();
        TomlLineGroup::KeyValue(word, expr)
    }

    fn eat_special(&mut self, target: TomlSpecialToken) -> Result<(), TomlAstError> {
        let token = self.tokens.next().ok_or(TomlAstError::Expect)?;
        match token.data() {
            TomlTokenData::Special(special) if *special == target => Ok(()),
            _ => Err(TomlAstError::Expect),
        }
    }

    fn parse_expr(mut self) -> Option<TomlExprIdx> {
        Some(self.exprs.alloc_one(match self.tokens.next()?.data() {
            TomlTokenData::Comment => todo!(),
            TomlTokenData::Special(_) => todo!(),
            TomlTokenData::Word(word) => match self.db.word_db().dt_coword(*word) {
                "true" => TomlExpr::Boolean(true),
                "false" => TomlExpr::Boolean(false),
                _ => todo!(),
            },
            TomlTokenData::StringLiteral { val, multiline: _ } => TomlExpr::String(val.clone()),
            TomlTokenData::Err(_) => todo!(),
        }))
    }
}
