use entity_route::{ScopeKind, *};
use file::FilePtr;
use text::{Row, TextRange};
use word::{BuiltinIdentifier, ContextualIdentifier, CustomIdentifier};

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
    Scope(ScopeKind),
    Variable { init_row: Row },
    Unrecognized(CustomIdentifier),
    ThisData { ty: Option<EntityRoutePtr> },
}

#[derive(Clone, Copy)]
pub struct SymbolProxy<'a> {
    pub(crate) main: Option<FilePtr>,
    pub(crate) db: &'a dyn AstSalsaQueryGroup,
    pub(crate) this_ty: Option<EntityRoutePtr>,
    pub(crate) symbols: &'a fold::LocalStack<Symbol>,
}

impl<'a> SymbolProxy<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: BuiltinIdentifier,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = Route::new_builtin(ident.into(), generics);
        let kind = AtomKind::Scope {
            scope: self.db.intern_scope(scope),
            kind: RawEntityKind::Type(match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::B32
                | BuiltinIdentifier::B64
                | BuiltinIdentifier::Bool => RawTyKind::Primitive,
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
                BuiltinIdentifier::CloneTrait => todo!(),
                BuiltinIdentifier::CopyTrait => todo!(),
                BuiltinIdentifier::PartialEqTrait => todo!(),
                BuiltinIdentifier::EqTrait => todo!(),
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
            Identifier::Contextual(ident) => match ident {
                ContextualIdentifier::Input => Ok(SymbolKind::Scope(ScopeKind::Contextual {
                    main: self
                        .main
                        .ok_or(error!(file, range, "can't use implicit without main"))?,
                    ident,
                })),
                ContextualIdentifier::ThisData => Ok(SymbolKind::ThisData { ty: self.this_ty }),
                ContextualIdentifier::ThisType => todo!(),
            },
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
        parent_scope: Route,
        subscope_ident: CustomIdentifier,
    ) -> Option<EntityRoutePtr> {
        self.db.subscope(
            self.db.intern_scope(parent_scope),
            subscope_ident,
            Vec::new(),
        )
    }
}
