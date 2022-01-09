use common::*;

use scope::*;
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Symbol {
    ident: word::Identifier,
    kind: SymbolKind,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Scope(scope::ScopeRoute),
    Variable(Range),
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolProxy<'a> {
    pub(crate) db: &'a dyn AstQuery,
    pub(crate) symbols: &'a fold::LocalStack<Symbol>,
    pub(crate) line: usize,
}

impl<'a> SymbolProxy<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: BuiltinIdentifier,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = Scope::builtin(ident.into(), generics);
        let kind = AtomKind::Scope(self.db.intern_scope(scope), ScopeKind::Type);
        Atom::new(tail, kind)
    }

    pub fn resolve_symbol_kind(
        &self,
        ident: Identifier,
        range: &TextRange,
    ) -> AstResult<SymbolKind> {
        match ident {
            Identifier::Builtin(builtin) => Ok(SymbolKind::Scope(builtin.into())),
            Identifier::Custom(ident) => Ok(self
                .symbols
                .find(|symbol| symbol.ident == ident.into())
                .ok_or(ast_error!(range.clone(), "what is this identifier?"))?
                .kind
                .clone()),
        }
    }

    fn resolve_subscope(
        &self,
        parent_scope: Scope,
        subscope_ident: CustomIdentifier,
    ) -> Option<Scope> {
        self.db.subscope(
            self.db.intern_scope(parent_scope),
            subscope_ident,
            Vec::new(),
        )
    }
}
