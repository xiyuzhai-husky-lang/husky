use scope::*;
use word::CustomIdentifier;

use crate::*;

pub struct SymbolDef {
    ident: word::Identifier,
    scope_id: scope::ScopeId,
}

#[derive(Debug, Clone, Copy)]
pub struct ScopeProxy<'a> {
    pub(crate) db: &'a dyn AtomQuery,
    pub(crate) symbols: &'a Vec<(CustomIdentifier, common::Range)>,
    pub(crate) line: usize,
}

impl<'a> ScopeProxy<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: BuiltinIdentifier,
        lifetimes: Vec<LifetimeParameter>,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = Scope::builtin(ident.into(), lifetimes, generics);
        let kind = AtomKind::Scope(self.db.intern_scope(scope), ScopeKind::Type);
        Atom::new(tail, kind)
    }

    pub fn resolve_scope_route(&self, token: &Token) -> Option<ScopeRoute> {
        match token.kind {
            TokenKind::Identifier(ident) => match ident {
                Identifier::Elide => None,
                Identifier::Builtin(builtin) => Some(builtin.into()),
                Identifier::Custom(ident) => self
                    .symbols
                    .iter()
                    .find(|(ident0, _)| ident == *ident0)
                    .map(|_| todo!()),
            },
            _ => None,
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
            Vec::new(),
        )
    }
}
