use file::FilePtr;
use scope::{ScopeRoute, *};
use text::{Row, TextRange};
use word::{BuiltinIdentifier, CustomIdentifier};

use super::*;
use crate::{query::AstSalsaQueryGroup, *};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ident: CustomIdentifier,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn var(ident: word::CustomIdentifier, init_row: Row) -> Self {
        Self {
            ident: ident.into(),
            kind: SymbolKind::Variable { init_row },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Scope(ScopeRoute),
    Variable { init_row: Row },
    Unrecognized(CustomIdentifier),
    This { ty: Option<ScopePtr> },
}

#[derive(Clone, Copy)]
pub struct SymbolProxy<'a> {
    pub(crate) main: Option<FilePtr>,
    pub(crate) db: &'a dyn AstSalsaQueryGroup,
    pub(crate) this: Option<ScopePtr>,
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
            kind: ScopeKind::Type(match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::B32
                | BuiltinIdentifier::B64
                | BuiltinIdentifier::Bool => TyKind::Primitive,
                BuiltinIdentifier::True => todo!(),
                BuiltinIdentifier::False => todo!(),
                BuiltinIdentifier::Vec => todo!(),
                BuiltinIdentifier::Tuple => todo!(),
                BuiltinIdentifier::Debug => todo!(),
                BuiltinIdentifier::Std => todo!(),
                BuiltinIdentifier::Core => todo!(),
                BuiltinIdentifier::Fp => todo!(),
                BuiltinIdentifier::Fn => todo!(),
                BuiltinIdentifier::FnMut => todo!(),
                BuiltinIdentifier::FnOnce => todo!(),
                BuiltinIdentifier::Array => todo!(),
                BuiltinIdentifier::DatasetType => todo!(),
                BuiltinIdentifier::Type => todo!(),
                BuiltinIdentifier::Datasets => todo!(),
            }),
        };
        Atom::new(tail, kind)
    }

    pub fn resolve_symbol_kind(
        &self,
        ident: Identifier,
        file: Option<FilePtr>,
        range: TextRange,
    ) -> AstResult<SymbolKind> {
        match ident {
            Identifier::Builtin(ident) => Ok(SymbolKind::Scope(ident.into())),
            Identifier::Implicit(ident) => Ok(SymbolKind::Scope(ScopeRoute::Implicit {
                main: self
                    .main
                    .ok_or(error!(file, range, "can't use implicit without main"))?,
                ident,
            })),
            Identifier::Custom(ident) => Ok(
                if let Some(symbol) = self.symbols.find(|symbol| symbol.ident == ident.into()) {
                    symbol.kind
                } else {
                    SymbolKind::Unrecognized(ident)
                },
            ),
            Identifier::This => Ok(SymbolKind::This { ty: self.this }),
        }
    }

    fn resolve_subscope(
        &self,
        parent_scope: Scope,
        subscope_ident: CustomIdentifier,
    ) -> Option<ScopePtr> {
        self.db.subscope(
            self.db.intern_scope(parent_scope),
            subscope_ident,
            Vec::new(),
        )
    }
}
