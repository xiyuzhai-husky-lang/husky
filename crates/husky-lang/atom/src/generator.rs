use folded::FoldedList;
use token::{Special, Token, TokenKind, TokenizedText};
use word::CustomIdentifier;

use crate::{
    query::AtomQuery,
    scope_proxy::{ScopeAliasStack, ScopeProxy},
    *,
};

pub struct AtomGenerator<'a> {
    db: &'a dyn AtomQuery,
    scope_aliases: ScopeAliasStack,
    folded_results: FoldedList<AtomParseResult>,
    symbols: Vec<(CustomIdentifier, common::Range)>,
}

impl<'a> AtomGenerator<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery, module: scope::Module) -> Self {
        Self {
            db,
            scope_aliases: ScopeAliasStack::new(),
            folded_results: FoldedList::new(),
            symbols: Vec::new(),
        }
    }

    pub(crate) fn take_folded_results(self) -> FoldedList<AtomParseResult> {
        self.folded_results
    }

    fn scope_proxy(&mut self) -> ScopeProxy {
        ScopeProxy {
            db: self.db,
            symbols: &self.symbols,
        }
    }
}

impl<'a> folded::Generator<'_, [Token], TokenizedText, AtomParseResult, AtomGenerator<'a>>
    for AtomGenerator<'a>
{
    fn enter_fold(&mut self) {
        self.scope_aliases.enter_fold();
    }

    fn exit_fold(&mut self) {
        self.scope_aliases.exit_fold();
    }

    fn transform(&mut self, tokens: &[Token]) -> AtomParseResult {
        let mut iter = tokens.iter().peekable();
        let atom_group = AtomGroup::new(
            if let TokenKind::Keyword(keyword) = iter.peek().unwrap().kind {
                iter.next();
                Some(keyword)
            } else {
                None
            },
            tokens.last().unwrap().kind == TokenKind::Special(Special::Colon),
        );
        parser::ScopeLRParser::new(self.scope_proxy(), tokens.into(), atom_group).parse()
    }

    fn folded_results(&mut self) -> &mut FoldedList<AtomParseResult> {
        &mut self.folded_results
    }
}
