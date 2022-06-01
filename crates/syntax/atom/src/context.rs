mod standalone;
mod symbol;

pub use standalone::*;
pub use symbol::*;

use super::*;
use defn_head::{GenericPlaceholderVariant, InputParameter, SpatialParameter};
use entity_kind::TyKind;
use entity_route::{EntityRouteKind, *};
use entity_syntax::{EntityRouteQueryGroup, EntitySyntaxResult};
use file::FilePtr;
use map_collect::MapCollect;
use print_utils::p;
use static_defn::{StaticGenericPlaceholder, StaticInputParameter};
use std::borrow::Cow;
use text::*;
use token::AbsSemanticToken;
use vm::InputLiason;
use word::{ContextualIdentifier, CustomIdentifier, IdentDict, RootIdentifier};

#[derive(Clone, Copy)]
pub enum AtomContextKind<'a> {
    Normal,
    Trait {
        this_trai: EntityRoutePtr,
        member_kinds: &'a [(CustomIdentifier, MemberKind)],
    },
}

#[derive(Default)]
pub struct AtomContextState {
    pub abs_semantic_tokens_len: usize,
}

pub trait AtomContext {
    fn opt_package_main(&self) -> Option<FilePtr>;
    fn entity_syntax_db(&self) -> &dyn EntityRouteQueryGroup;
    fn opt_this_ty(&self) -> Option<EntityRoutePtr>;
    fn opt_this_liason(&self) -> Option<InputLiason>;
    fn symbols(&self) -> &[Symbol];
    fn kind(&self) -> AtomContextKind;
    fn as_dyn_mut(&mut self) -> &mut dyn AtomContext;
    fn push_abs_semantic_token(&mut self, new_token: AbsSemanticToken);
    fn save_state(&self) -> AtomContextState;
    fn rollback(&mut self, state: AtomContextState);
    fn push_symbol(&mut self, new_symbol: Symbol);

    fn builtin_type_atom(
        &self,
        ident: RootIdentifier,
        generics: Vec<SpatialArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = EntityRoute::new_root(ident.into(), generics);
        let kind = AtomVariant::EntityRoute {
            route: self.entity_syntax_db().intern_entity_route(scope),
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
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::Datasets => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::ModuleType => todo!(),
            }),
        };
        Atom::new(tail, kind)
    }

    fn resolve_symbol_kind(&self, ident: Identifier, range: TextRange) -> AtomResult<SymbolKind> {
        match ident {
            Identifier::Builtin(ident) => Ok(SymbolKind::EntityRoute(ident.into())),
            Identifier::Contextual(ident) => match ident {
                ContextualIdentifier::Input => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db().intern_entity_route(EntityRoute {
                        kind: EntityRouteKind::Input {
                            main: self
                                .opt_package_main()
                                .ok_or(error!("can't use implicit without main", range))?,
                        },
                        spatial_arguments: vec![],
                    }),
                )),
                ContextualIdentifier::ThisValue => Ok(SymbolKind::ThisValue {
                    opt_this_ty: self.opt_this_ty(),
                    opt_this_liason: self.opt_this_liason(),
                }),
                ContextualIdentifier::ThisType => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db().entity_route_menu().this_ty,
                )),
                ContextualIdentifier::Crate => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db()
                        .module(self.opt_package_main().unwrap())
                        .unwrap(),
                )),
            },
            Identifier::Custom(ident) => Ok(if let Some(symbol) = self.find_symbol(ident) {
                symbol.kind
            } else {
                SymbolKind::Unrecognized(ident)
            }),
        }
    }

    fn find_symbol(&self, ident: CustomIdentifier) -> Option<&Symbol> {
        self.symbols()
            .iter()
            .rev()
            .find(|symbol| symbol.ident == ident)
    }

    fn entity_kind(&self, route: EntityRoutePtr, range: TextRange) -> AtomResult<EntityKind> {
        let kind_result: EntitySyntaxResult<EntityKind> = match route.kind {
            EntityRouteKind::Child {
                parent,
                ident: ident0,
            } => match parent.kind {
                EntityRouteKind::ThisType => match self.kind() {
                    AtomContextKind::Normal => todo!(),
                    AtomContextKind::Trait {
                        member_kinds: members,
                        ..
                    } => {
                        match members
                            .iter()
                            .find(|(ident, _)| *ident == ident0)
                            .unwrap()
                            .1
                        {
                            MemberKind::Method => todo!(),
                            MemberKind::Call => todo!(),
                            MemberKind::TraitAssociatedType => Ok(EntityKind::Type(TyKind::Other)),
                            MemberKind::TraitAssociatedConstSize => todo!(),
                            MemberKind::Field => todo!(),
                            MemberKind::TraitAssociatedAny => panic!(),
                        }
                    }
                },
                _ => self.entity_syntax_db().entity_kind(route),
            },
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => todo!(),
            _ => self.entity_syntax_db().entity_kind(route),
        };
        match kind_result {
            Ok(kind) => Ok(kind),
            Err(e) => err!(e.message, range),
        }
    }

    fn entity_route_from_str(&mut self, text: &str) -> AtomResult<EntityRoutePtr> {
        let tokens = self.entity_syntax_db().tokenize(text);
        let result =
            AtomParser::new(self.as_dyn_mut(), &mut (&tokens as &[_]).into()).parse_all()?;
        if result.len() == 0 {
            panic!()
        }
        if result.len() > 1 {
            p!(result);
            err!("too many atoms", result[1..].text_range())?
        } else {
            match result[0].kind {
                AtomVariant::EntityRoute { route: scope, .. } => Ok(scope),
                // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
                _ => err!(
                    format!("expect type, but get `{:?}` instead", result[0]),
                    (&result).text_range()
                )?,
            }
        }
    }

    fn input_placeholder_from_static(
        &mut self,
        static_input_placeholder: &StaticInputParameter,
    ) -> InputParameter {
        InputParameter {
            ranged_ident: RangedCustomIdentifier {
                ident: self
                    .entity_syntax_db()
                    .intern_word(static_input_placeholder.name)
                    .custom(),
                range: Default::default(),
            },
            liason: static_input_placeholder.contract,
            ranged_ty: RangedEntityRoute {
                route: self
                    .entity_route_from_str(static_input_placeholder.ty)
                    .unwrap(),
                range: Default::default(),
            },
        }
    }

    fn trai(&self) -> EntityRoutePtr {
        match self.kind() {
            AtomContextKind::Normal => panic!(),
            AtomContextKind::Trait {
                this_trai: trai, ..
            } => trai,
        }
    }

    fn generic_parameters_from_static(
        &self,
        static_generic_parameters: &[StaticGenericPlaceholder],
    ) -> IdentDict<SpatialParameter> {
        static_generic_parameters.map(|static_generic_placeholder| SpatialParameter {
            ident: self
                .entity_syntax_db()
                .intern_word(static_generic_placeholder.name)
                .custom(),
            variant: GenericPlaceholderVariant::Type { traits: vec![] },
        })
    }

    fn generic_arguments_from_generic_parameters(
        &self,
        generic_parameters: &[SpatialParameter],
    ) -> Vec<SpatialArgument> {
        generic_parameters.map(|generic_placeholder| {
            SpatialArgument::EntityRoute(self.entity_syntax_db().intern_entity_route(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                spatial_arguments: vec![],
            }))
        })
    }

    fn symbols_from_generic_parameters(
        &self,
        generic_parameters: &[SpatialParameter],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for generic_placeholder in generic_parameters.iter() {
            symbols.push(Symbol {
                ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.entity_syntax_db().intern_entity_route(
                    EntityRoute {
                        kind: EntityRouteKind::Generic {
                            ident: generic_placeholder.ident,
                            entity_kind: generic_placeholder.entity_kind(),
                        },
                        spatial_arguments: vec![],
                    },
                )),
            })
        }
        symbols
    }
}
