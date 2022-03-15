use common::*;

use file::FilePtr;
use scope::{ScopeRoute, *};
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use super::*;
use crate::{query::AstSalsaQueryGroup, *};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ident: CustomIdentifier,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn var(ident: word::CustomIdentifier, range: TextRange) -> Self {
        Self {
            ident: ident.into(),
            kind: SymbolKind::Variable(range),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Scope(ScopeRoute),
    Variable(TextRange),
    Unrecognized(CustomIdentifier),
}

#[derive(Clone, Copy)]
pub struct SymbolProxy<'a> {
    pub(crate) main: Option<FilePtr>,
    pub(crate) db: &'a dyn AstSalsaQueryGroup,
    pub(crate) symbols: &'a fold::LocalStack<Symbol>,
}

impl<'a> SymbolProxy<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: BuiltinIdentifier,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = Scope::new_builtin(ident.into(), generics);
        let kind = AtomKind::Scope {
            scope: self.db.intern_scope(scope),
            kind: ScopeKind::Type,
        };
        Atom::new(tail, kind)
    }

    pub fn resolve_symbol_kind(
        &self,
        ident: Identifier,
        range: TextRange,
    ) -> AstResult<SymbolKind> {
        match ident {
            Identifier::Builtin(ident) => Ok(SymbolKind::Scope(ident.into())),
            Identifier::Implicit(ident) => Ok(SymbolKind::Scope(ScopeRoute::Implicit {
                main: self
                    .main
                    .ok_or(error!(range, "can't use implicit outside package"))?,
                ident,
            })),
            Identifier::Custom(ident) => Ok(
                if let Some(symbol) = self.symbols.find(|symbol| symbol.ident == ident.into()) {
                    symbol.kind
                } else {
                    SymbolKind::Unrecognized(ident)
                },
            ),
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
