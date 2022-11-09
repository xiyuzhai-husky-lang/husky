mod standalone;
mod symbol;

pub use standalone::*;
pub use symbol::*;

use super::*;
use husky_defn_head::{Parameter, SpatialParameter, SpatialParameterVariant};
use husky_entity_kind::TyKind;
use husky_entity_route::{EntityRouteVariant, *};
use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxResult};
use husky_file::{FileItd, FileSalsaQuery};
use husky_print_utils::p;
use husky_static_defn::{StaticParameter, StaticSpatialParameter};
use husky_text::*;
use husky_token::AbsSemanticToken;
use husky_word::{ContextualIdentifier, CustomIdentifier, IdentDict, RootBuiltinIdentifier};
use map_collect::MapCollect;
use std::borrow::Cow;
use thin_vec::{thin_vec, ThinVec};

#[derive(Clone, Copy)]
pub enum AtomContextKind<'a> {
    Normal,
    Trait {
        this_trai: EntityRouteItd,
        member_kinds: &'a [(CustomIdentifier, MemberKind)],
    },
}

#[derive(Default)]
pub struct AtomContextState {
    pub abs_semantic_tokens_len: usize,
}

pub trait AtomContext<'a> {
    fn file(&self) -> FileItd;
    fn entity_syntax_db(&self) -> &'a dyn EntitySyntaxQueryGroup;
    fn opt_target_entrance(&self) -> Option<FileItd> {
        FileSalsaQuery::opt_target_entrance(self.entity_syntax_db())
    }
    fn opt_this_ty(&self) -> Option<EntityRouteItd>;
    fn opt_this_liason(&self) -> Option<ParameterModifier>;
    fn symbols(&self) -> &[Symbol];
    fn kind(&self) -> AtomContextKind;
    fn as_dyn_mut(&mut self) -> &mut dyn AtomContext<'a>;
    fn push_abs_semantic_token(&mut self, new_token: AbsSemanticToken);
    fn save_state(&self) -> AtomContextState;
    fn rollback(&mut self, state: AtomContextState);
    fn push_symbol(&mut self, new_symbol: Symbol);

    fn push_new_symbol(&mut self, new: Symbol) -> Option<Symbol> {
        let old = self
            .symbols()
            .iter()
            .find(|old| old.init_ident == new.init_ident)
            .map(|s| s.clone());
        self.push_symbol(new);
        old
    }

    fn builtin_type_atom(
        &self,
        ident: RootBuiltinIdentifier,
        generics: ThinVec<SpatialArgument>,
        tail: TextRange,
    ) -> HuskyAtom {
        let scope = EntityRoute::new_root(ident.into(), generics);
        let kind = HuskyAtomVariant::EntityRoute {
            route: self.entity_syntax_db().intern_entity_route(scope),
            kind: EntityKind::Type(match ident {
                RootBuiltinIdentifier::Void
                | RootBuiltinIdentifier::I32
                | RootBuiltinIdentifier::I64
                | RootBuiltinIdentifier::F32
                | RootBuiltinIdentifier::F64
                | RootBuiltinIdentifier::B32
                | RootBuiltinIdentifier::B64
                | RootBuiltinIdentifier::Bool => TyKind::Primitive,
                RootBuiltinIdentifier::True => todo!(),
                RootBuiltinIdentifier::False => todo!(),
                RootBuiltinIdentifier::Vec => todo!(),
                RootBuiltinIdentifier::Tuple => TyKind::Tuple,
                RootBuiltinIdentifier::Debug => todo!(),
                RootBuiltinIdentifier::Std => todo!(),
                RootBuiltinIdentifier::Core => todo!(),
                RootBuiltinIdentifier::Mor => TyKind::Mor,
                RootBuiltinIdentifier::ThickFp => TyKind::ThickFp,
                RootBuiltinIdentifier::Fn => todo!(),
                RootBuiltinIdentifier::FnMut => todo!(),
                RootBuiltinIdentifier::FnOnce => todo!(),
                RootBuiltinIdentifier::Array => todo!(),
                RootBuiltinIdentifier::DatasetType => todo!(),
                RootBuiltinIdentifier::TypeType => todo!(),
                RootBuiltinIdentifier::Trait => todo!(),
                RootBuiltinIdentifier::Domains => todo!(),
                RootBuiltinIdentifier::CloneTrait => todo!(),
                RootBuiltinIdentifier::CopyTrait => todo!(),
                RootBuiltinIdentifier::PartialEqTrait => todo!(),
                RootBuiltinIdentifier::EqTrait => todo!(),
                RootBuiltinIdentifier::Module => todo!(),
                RootBuiltinIdentifier::Ref => todo!(),
                RootBuiltinIdentifier::RefMut => todo!(),
                RootBuiltinIdentifier::Option => todo!(),
                RootBuiltinIdentifier::VisualType => todo!(),
            }),
        };
        HuskyAtom::new(tail, kind)
    }

    fn resolve_symbol_kind(&self, ident: Identifier, range: TextRange) -> AtomResult<SymbolKind> {
        match ident {
            Identifier::Root(ident) => Ok(SymbolKind::EntityRoute(ident.into())),
            Identifier::Contextual(ident) => match ident {
                ContextualIdentifier::CrateInputValue => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db().intern_entity_route(EntityRoute {
                        variant: EntityRouteVariant::TargetInputValue,
                        temporal_arguments: thin_vec![],
                        spatial_arguments: thin_vec![],
                    }),
                )),
                ContextualIdentifier::TargetOutputType => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db().intern_entity_route(EntityRoute {
                        variant: EntityRouteVariant::TargetOutputType,
                        temporal_arguments: thin_vec![],
                        spatial_arguments: thin_vec![],
                    }),
                )),
                ContextualIdentifier::ThisValue => Ok(SymbolKind::ThisValue {
                    opt_this_ty: self.opt_this_ty(),
                    opt_this_liason: self.opt_this_liason(),
                }),
                ContextualIdentifier::ThisType => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db().intern_entity_route(EntityRoute {
                        // ad hoc
                        variant: EntityRouteVariant::ThisType {
                            file: self.file(),
                            range,
                        },
                        temporal_arguments: Default::default(),
                        spatial_arguments: Default::default(),
                    }),
                )),
                ContextualIdentifier::Crate => Ok(SymbolKind::EntityRoute(
                    self.entity_syntax_db()
                        .module(self.opt_target_entrance().unwrap())
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
            .find(|symbol| symbol.init_ident.ident == ident)
    }

    fn entity_kind(&self, route: EntityRouteItd, range: TextRange) -> AtomResult<EntityKind> {
        let kind_result: EntitySyntaxResult<EntityKind> = match route.variant {
            EntityRouteVariant::Child {
                parent,
                ident: ident0,
            } => match parent.variant {
                EntityRouteVariant::ThisType { .. } => match self.kind() {
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
                            MemberKind::Method { .. } => todo!(),
                            MemberKind::Call => todo!(),
                            MemberKind::TraitAssociatedType => {
                                Ok(EntityKind::Type(TyKind::AssociatedAny))
                            }
                            MemberKind::TraitAssociatedConstSize => todo!(),
                            MemberKind::Field => todo!(),
                            MemberKind::TraitAssociatedAny => panic!(),
                        }
                    }
                },
                _ => self.entity_syntax_db().husky_entity_kind(route),
            },
            EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
            _ => self.entity_syntax_db().husky_entity_kind(route),
        };
        match kind_result {
            Ok(kind) => Ok(kind),
            Err(e) => err!(e.message, range),
        }
    }

    fn parse_entity_route(&mut self, text: &str) -> AtomResult<EntityRouteItd> {
        let tokens = self.entity_syntax_db().tokenize_line(text);
        let result = AtomParser::new(self.as_dyn_mut(), &mut (&tokens as &[_]).into())
            .parse_all_remaining_atoms()?;
        if result.len() == 0 {
            panic!()
        }
        if result.len() > 1 {
            p!(result);
            err!("too many atoms", result[1..].text_range())?
        } else {
            match result[0].variant {
                HuskyAtomVariant::EntityRoute { route, .. } => Ok(route),
                // AtomKind::ThisType { ty } => Ok(EntityRoutePtr::ThisType),
                _ => err!(
                    format!("expect type, but get `{:?}` instead", result[0]),
                    (&result).text_range()
                )?,
            }
        }
    }

    fn parameter_from_static(&mut self, static_parameter: &StaticParameter) -> Parameter {
        Parameter::new(
            self.entity_syntax_db(),
            RangedCustomIdentifier {
                ident: self
                    .entity_syntax_db()
                    .it_word(static_parameter.name)
                    .custom(),
                range: Default::default(),
            },
            static_parameter.modifier.into(),
            RangedEntityRoute {
                route: self.parse_entity_route(static_parameter.ty).unwrap(),
                range: Default::default(),
            },
        )
    }

    fn trai(&self) -> EntityRouteItd {
        match self.kind() {
            AtomContextKind::Normal => panic!(),
            AtomContextKind::Trait {
                this_trai: trai, ..
            } => trai,
        }
    }

    fn generic_parameters_from_static(
        &self,
        static_generic_parameters: &[StaticSpatialParameter],
    ) -> IdentDict<SpatialParameter> {
        static_generic_parameters.map(|static_spatial_parameter| SpatialParameter {
            ident: RangedCustomIdentifier {
                ident: self
                    .entity_syntax_db()
                    .it_word(static_spatial_parameter.name)
                    .custom(),
                range: Default::default(),
            },
            variant: SpatialParameterVariant::Type { traits: vec![] },
            file: self
                .entity_syntax_db()
                .intern_file(static_spatial_parameter.dev_src.file.into()),
            range: TextRange::from_line(static_spatial_parameter.dev_src.line),
        })
    }

    fn generic_arguments_from_generic_parameters(
        &self,
        generic_parameters: &[SpatialParameter],
    ) -> ThinVec<SpatialArgument> {
        generic_parameters.map(|spatial_parameter| {
            SpatialArgument::EntityRoute(self.entity_syntax_db().intern_entity_route(EntityRoute {
                variant: EntityRouteVariant::Any {
                    ident: spatial_parameter.ident.ident,
                    husky_entity_kind: spatial_parameter.husky_entity_kind(),
                    file: spatial_parameter.file,
                    range: spatial_parameter.range,
                },
                temporal_arguments: thin_vec![],
                spatial_arguments: thin_vec![],
            }))
        })
    }

    fn symbols_from_generic_parameters(
        &self,
        spatial_parameters: &[SpatialParameter],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for spatial_parameter in spatial_parameters.iter() {
            symbols.push(Symbol {
                init_ident: spatial_parameter.ident,
                kind: SymbolKind::EntityRoute(self.entity_syntax_db().intern_entity_route(
                    EntityRoute {
                        variant: EntityRouteVariant::Any {
                            ident: spatial_parameter.ident.ident,
                            husky_entity_kind: spatial_parameter.husky_entity_kind(),
                            file: spatial_parameter.file,
                            range: spatial_parameter.range,
                        },
                        temporal_arguments: thin_vec![],
                        spatial_arguments: thin_vec![],
                    },
                )),
            })
        }
        symbols
    }
}
