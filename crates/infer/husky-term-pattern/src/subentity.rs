use crate::*;
use husky_word::Identifier;

pub struct TermSubentityPattern<'a> {
    parent: &'a TermPattern<'a>,
    ident: Identifier,
}
