use std::borrow::Cow;

use defn_head::{GenericPlaceholder, GenericPlaceholderVariant, InputPlaceholder};
use entity_kind::TyKind;
use entity_route::{EntityRouteKind, *};
use entity_route_query::EntityRouteQueryGroup;
use file::FilePtr;
use map_collect::MapCollect;
use print_utils::p;
use static_defn::{StaticGenericPlaceholder, StaticInputPlaceholder};
use text::{Row, TextRange};
use word::{ContextualIdentifier, CustomIdentifier, IdentDict, RootIdentifier};

use super::*;

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
    EntityRoute(EntityRoutePtr),
    Variable { init_row: Row },
    Unrecognized(CustomIdentifier),
    ThisData { ty: Option<EntityRoutePtr> },
}

#[derive(Clone)]
pub struct SymbolContext<'a> {
    pub opt_package_main: Option<FilePtr>,
    pub db: &'a dyn EntityRouteQueryGroup,
    pub opt_this_ty: Option<EntityRoutePtr>,
    // pub this_ty_members: Option<EntityRoute>,
    pub symbols: Cow<'a, [Symbol]>,
    pub kind: SymbolContextKind<'a>,
}

#[derive(Clone, Copy)]
pub enum SymbolContextKind<'a> {
    Normal,
    Trait {
        trai: EntityRoutePtr,
        members: &'a [(CustomIdentifier, MemberKind)],
    },
}

impl<'a> SymbolContext<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: RootIdentifier,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = EntityRoute::new_builtin(ident.into(), generics);
        let kind = AtomKind::EntityRoute {
            route: self.db.intern_entity_route(scope),
            kind: EntityKind::Type(match ident {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::F32
                | RootIdentifier::B32
                | RootIdentifier::B64
                | RootIdentifier::Bool => TyKind::Primitive,
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => TyKind::Other,
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Fp => TyKind::Other,
                RootIdentifier::Fn => TyKind::Other,
                RootIdentifier::FnMut => TyKind::Other,
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::Type => todo!(),
                RootIdentifier::Datasets => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
            }),
        };
        Atom::new(tail, kind)
    }

    pub fn resolve_symbol_kind(
        &self,
        ident: Identifier,
        range: TextRange,
    ) -> AtomResult<SymbolKind> {
        match ident {
            Identifier::Builtin(ident) => Ok(SymbolKind::EntityRoute(ident.into())),
            Identifier::Contextual(ident) => match ident {
                ContextualIdentifier::Input => Ok(SymbolKind::EntityRoute(
                    self.db.intern_entity_route(EntityRoute {
                        kind: EntityRouteKind::Input {
                            main: self
                                .opt_package_main
                                .ok_or(error!("can't use implicit without main", range))?,
                        },
                        generic_arguments: vec![],
                    }),
                )),
                ContextualIdentifier::ThisData => Ok(SymbolKind::ThisData {
                    ty: self.opt_this_ty,
                }),
                ContextualIdentifier::ThisType => {
                    Ok(SymbolKind::EntityRoute(self.db.entity_route_menu().this_ty))
                }
            },
            Identifier::Custom(ident) => Ok(if let Some(symbol) = self.find_symbol(ident) {
                symbol.kind
            } else {
                SymbolKind::Unrecognized(ident)
            }),
        }
    }

    fn find_symbol(&self, ident: CustomIdentifier) -> Option<&Symbol> {
        self.symbols.iter().find(|symbol| symbol.ident == ident)
    }

    pub fn entity_kind(&self, route: EntityRoutePtr) -> EntityKind {
        match route.kind {
            EntityRouteKind::Child {
                parent,
                ident: ident0,
            } => match parent.kind {
                EntityRouteKind::ThisType => match self.kind {
                    SymbolContextKind::Normal => todo!(),
                    SymbolContextKind::Trait { members, .. } => {
                        match members
                            .iter()
                            .find(|(ident, _)| *ident == ident0)
                            .unwrap()
                            .1
                        {
                            MemberKind::Method => todo!(),
                            MemberKind::Call => todo!(),
                            MemberKind::AssociatedType => EntityKind::Type(TyKind::Other),
                            MemberKind::AssociatedConstSize => todo!(),
                        }
                    }
                },
                _ => self.db.raw_entity_kind(route),
            },
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => todo!(),
            _ => self.db.raw_entity_kind(route),
        }
    }

    pub fn entity_route_from_str(&self, text: &str) -> AtomResult<EntityRoutePtr> {
        let tokens = self.db.tokenize(text);
        let result = AtomLRParser::new(self, &tokens).parse_all()?;
        if result.len() == 0 {
            panic!()
        }
        if result.len() > 1 {
            p!(result);
            err!("too many atoms", result[1..].into())?
        } else {
            match result[0].kind {
                AtomKind::EntityRoute { route: scope, .. } => Ok(scope),
                // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
                _ => err!(
                    format!("expect type, but get `{:?}` instead", result[0]),
                    (&result).into()
                )?,
            }
        }
    }

    pub fn input_placeholder(
        &self,
        static_input_placeholder: &StaticInputPlaceholder,
    ) -> InputPlaceholder {
        InputPlaceholder {
            ident: self.db.intern_word(static_input_placeholder.name).custom(),
            contract: static_input_placeholder.contract,
            ranged_ty: RangedEntityRoute {
                route: self
                    .entity_route_from_str(static_input_placeholder.ty)
                    .unwrap(),
                range: Default::default(),
            },
        }
    }

    pub fn trai(&self) -> EntityRoutePtr {
        match self.kind {
            SymbolContextKind::Normal => panic!(),
            SymbolContextKind::Trait { trai, .. } => trai,
        }
    }

    pub fn generic_placeholders_from_static(
        &self,
        static_generic_placeholders: &[StaticGenericPlaceholder],
    ) -> IdentDict<GenericPlaceholder> {
        static_generic_placeholders.map(|static_generic_placeholder| GenericPlaceholder {
            ident: self
                .db
                .intern_word(static_generic_placeholder.name)
                .custom(),
            variant: GenericPlaceholderVariant::Type { traits: vec![] },
        })
    }

    pub fn generic_arguments_from_generic_placeholders(
        &self,
        generic_placeholders: &[GenericPlaceholder],
    ) -> Vec<GenericArgument> {
        generic_placeholders.map(|generic_placeholder| {
            GenericArgument::EntityRoute(self.db.intern_entity_route(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                generic_arguments: vec![],
            }))
        })
    }

    pub fn symbols_from_generic_placeholders(
        &self,
        generic_placeholders: &[GenericPlaceholder],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for generic_placeholder in generic_placeholders.iter() {
            symbols.push(Symbol {
                ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.db.intern_entity_route(EntityRoute {
                    kind: EntityRouteKind::Generic {
                        ident: generic_placeholder.ident,
                        entity_kind: generic_placeholder.entity_kind(),
                    },
                    generic_arguments: vec![],
                })),
            })
        }
        symbols
    }
}
