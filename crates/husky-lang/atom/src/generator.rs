use folded::FoldedList;
use token::{Special, Token, TokenKind, TokenizedText};
use word::CustomIdentifier;

use crate::{query::AtomQuery, scope_proxy::ScopeProxy, *};

pub struct AtomGenerator<'a> {
    db: &'a dyn AtomQuery,
    folded_results: FoldedList<AtomParseResult>,
    symbols: Vec<(CustomIdentifier, common::Range)>,
}

impl<'a> AtomGenerator<'a> {
    pub(crate) fn new(db: &'a dyn AtomQuery, module: scope::Module) -> Self {
        Self {
            db,
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
            line: 0,
        }
    }
}

impl<'a> folded::Generator<'_, [Token], TokenizedText, AtomParseResult> for AtomGenerator<'a> {
    fn enter_fold(&mut self) {}

    fn exit_fold(&mut self) {}

    fn transform(&mut self, tokens: &[Token]) -> AtomParseResult {
        if let TokenKind::Keyword(keyword) = tokens[0].kind {
            match keyword {
                Keyword::Func(_) => todo!(),
                Keyword::Type(_) => todo!(),
                Keyword::Use | Keyword::Mod => todo!(),
                Keyword::Stmt(kw) => {
                    let is_head = tokens.last().unwrap().kind == TokenKind::Special(Special::Colon);
                    let end = if is_head {
                        tokens.len() - 1
                    } else {
                        tokens.len()
                    };
                    AtomicLineGroup::stmt(self.scope_proxy(), Some(kw), is_head, &tokens[1..end])
                }
            }
        } else {
            AtomicLineGroup::stmt(self.scope_proxy(), None, false, tokens)
        }
    }

    fn folded_results(&mut self) -> &mut FoldedList<AtomParseResult> {
        &mut self.folded_results
    }
}
