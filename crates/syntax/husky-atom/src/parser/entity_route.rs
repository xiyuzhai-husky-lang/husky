use super::context::SymbolKind;
use super::*;
use entity_kind::TyKind;
use husky_entity_route::{entity_route_menu, make_subroute, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use husky_token::{is_special, special_token, SemanticTokenKind};
use thin_vec::{thin_vec, ThinVec};

/// parse atoms from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn symbol(&mut self) -> AtomResult<Option<HuskyAtomVariant>> {
        Ok(if let Some(token) = self.token_stream.next() {
            if is_special!(token, "[") {
                self.atom_context
                    .push_abs_semantic_token(AbsSemanticToken::new(
                        SemanticTokenKind::Special,
                        token.range,
                    ));
                let (route, ty_kind) = if try_eat!(self, SpecialToken::RBox) {
                    (self.vec_ty()?, TyKind::Vec)
                } else if try_eat!(self, SpecialToken::Modulo) {
                    eat!(self, token_kind, SpecialToken::RBox.into());
                    let element = self.spatial_argument()?;
                    (
                        entity_route_menu().std_slice_cyclic_slice.call([element]),
                        TyKind::CyclicSlice,
                    )
                } else if let Some(size) = try_get!(self, usize_literal) {
                    if !try_eat!(self, special, SpecialToken::RBox) {
                        return Ok(None);
                    }
                    if let Some(token) = self.token_stream.peek() {
                        match token.left_convexity() {
                            Some(Convexity::Convex) => (
                                EntityRoute::array(self.spatial_argument()?, size),
                                TyKind::Array,
                            ),
                            _ => return Ok(None),
                        }
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Ok(None);
                };
                Some(HuskyAtomVariant::EntityRoute {
                    route: self
                        .atom_context
                        .entity_syntax_db()
                        .intern_entity_route(route),
                    kind: EntityKind::Type(ty_kind),
                })
            } else if is_special!(token, "&") {
                let ty = get!(self, ty?);
                Some(HuskyAtomVariant::EntityRoute {
                    route: self
                        .db()
                        .route_call(RootIdentifier::Ref.into(), thin_vec![ty.into()]),
                    kind: EntityKind::Type(TyKind::Ref),
                })
            } else if is_special!(token, "?") {
                let ty = get!(self, ty?);
                Some(HuskyAtomVariant::EntityRoute {
                    route: self
                        .db()
                        .route_call(RootIdentifier::Option.into(), thin_vec![ty.into()]),
                    kind: EntityKind::Type(TyKind::Option),
                })
            } else if let HuskyTokenKind::Identifier(ident) = token.kind {
                let symbol_kind = self.atom_context.resolve_symbol_kind(ident, token.range)?;
                Some(match symbol_kind {
                    SymbolKind::EntityRoute(route) => {
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::Entity(
                                    self.atom_context.entity_kind(route, token.range)?,
                                ),
                                token.range,
                            ));
                        self.normal_route(route)?
                    }
                    SymbolKind::Variable { init_range } => {
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::Variable,
                                token.range,
                            ));
                        match ident {
                            Identifier::Builtin(_) | Identifier::Contextual(_) => panic!(),
                            Identifier::Custom(varname) => HuskyAtomVariant::Variable {
                                varname,
                                init_range,
                            },
                        }
                    }
                    SymbolKind::Unrecognized(ident) => HuskyAtomVariant::Unrecognized(ident),
                    SymbolKind::ThisValue {
                        opt_this_ty,
                        opt_this_liason,
                    } => {
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::ThisValue,
                                token.range,
                            ));
                        HuskyAtomVariant::ThisValue {
                            opt_this_ty,
                            opt_this_liason,
                        }
                    }
                    SymbolKind::ThisField {
                        opt_this_ty,
                        opt_field_ty,
                        field_liason,
                    } => {
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::Field,
                                token.range,
                            ));
                        HuskyAtomVariant::ThisField {
                            field_ident: RangedCustomIdentifier {
                                ident: ident.custom(),
                                range: token.range,
                            },
                            opt_this_ty,
                            opt_this_liason: self.atom_context.opt_this_liason(),
                            opt_field_ty,
                            field_liason,
                        }
                    }
                    SymbolKind::FrameVariable { init_range } => {
                        self.atom_context
                            .push_abs_semantic_token(AbsSemanticToken::new(
                                SemanticTokenKind::FrameVariable,
                                token.range,
                            ));
                        HuskyAtomVariant::FrameVariable {
                            varname: ident.custom(),
                            init_range,
                        }
                    }
                    SymbolKind::ThisMethod => {
                        p!(self.atom_context.opt_target_entrance(), token.range);
                        todo!()
                    }
                })
            } else {
                None
            }
        } else {
            None
        })
    }

    fn vec_ty(&mut self) -> AtomResult<EntityRoute> {
        Ok(EntityRoute::vec(self.spatial_argument()?))
    }

    fn normal_route(&mut self, route: EntityRoutePtr) -> AtomResult<HuskyAtomVariant> {
        let generic_arguments = self.generics(route)?;
        let mut route = self
            .atom_context
            .entity_syntax_db()
            .route_call(route, generic_arguments);
        while try_eat!(self, SpecialToken::DoubleColon) {
            let ranged_ident = get!(self, custom_ident);
            let new_route = make_subroute(route, ranged_ident.ident, Default::default());
            match self.atom_context.entity_syntax_db().entity_kind(new_route) {
                Ok(_) => (),
                Err(e) => {
                    let message = e.message;
                    err!(format!("{message}"), ranged_ident.range)?
                }
            }
            let generic_arguments = self.generics(new_route)?;
            route = make_subroute(route, ranged_ident.ident, generic_arguments);
            self.atom_context
                .push_abs_semantic_token(AbsSemanticToken::new(
                    SemanticTokenKind::Entity(
                        self.atom_context.entity_kind(route, ranged_ident.range)?,
                    ),
                    ranged_ident.range,
                ));
        }
        return Ok(HuskyAtomVariant::EntityRoute {
            route,
            kind: self
                .atom_context
                .entity_kind(route, Default::default())
                .unwrap(),
        });
    }

    pub(crate) fn ty(&mut self) -> AtomResult<Option<EntityRoutePtr>> {
        Ok(
            if let Some(HuskyAtomVariant::EntityRoute { route, kind, .. }) = self.symbol()? {
                if let EntityKind::Type(_) = kind {
                    Some(route)
                } else {
                    None
                }
            } else {
                None
            },
        )
    }

    pub fn ranged_ty(&mut self) -> AtomResult<Option<RangedEntityRoute>> {
        let text_start = self.token_stream.text_start();
        Ok(
            if let Some(HuskyAtomVariant::EntityRoute { route, kind, .. }) = self.symbol()? {
                if let EntityKind::Type(_) = kind {
                    Some(RangedEntityRoute {
                        route,
                        range: self.token_stream.text_range(text_start),
                    })
                } else {
                    None
                }
            } else {
                None
            },
        )
    }

    fn generics(&mut self, route: EntityRoutePtr) -> AtomResult<ThinVec<SpatialArgument>> {
        if route.spatial_arguments.len() > 0 {
            p!(route);
            todo!()
        }
        match route.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::I64
                | RootIdentifier::F32
                | RootIdentifier::F64
                | RootIdentifier::B32
                | RootIdentifier::B64
                | RootIdentifier::Bool
                | RootIdentifier::True
                | RootIdentifier::False
                | RootIdentifier::Debug
                | RootIdentifier::Std
                | RootIdentifier::Core
                | RootIdentifier::Domains
                | RootIdentifier::CloneTrait
                | RootIdentifier::CopyTrait
                | RootIdentifier::PartialEqTrait
                | RootIdentifier::EqTrait
                | RootIdentifier::TypeType
                | RootIdentifier::TraitType
                | RootIdentifier::ModuleType => Ok(thin_vec![]),
                RootIdentifier::Mor
                | RootIdentifier::Fp
                | RootIdentifier::Fn
                | RootIdentifier::FnMut
                | RootIdentifier::FnOnce => Ok(self.func_args()?),
                RootIdentifier::Vec
                | RootIdentifier::Array
                | RootIdentifier::Tuple
                | RootIdentifier::DatasetType
                | RootIdentifier::Ref
                | RootIdentifier::Option => self.angled_generics(),
                RootIdentifier::VisualType => todo!(),
            },
            _ => {
                let entity_kind = self
                    .atom_context
                    .entity_kind(route, Default::default())
                    .unwrap();
                match entity_kind {
                    EntityKind::Module
                    | EntityKind::EnumVariant
                    | EntityKind::Feature
                    | EntityKind::Member(_) => Ok(thin_vec![]),
                    EntityKind::Type(_) | EntityKind::Trait | EntityKind::Function { .. } => {
                        self.angled_generics()
                    }
                    EntityKind::Main => panic!(),
                }
            }
        }
    }

    fn func_args(&mut self) -> AtomResult<ThinVec<SpatialArgument>> {
        if !try_eat!(self, "(") {
            return Ok(Default::default());
        }
        let mut args = thin_comma_list![self, spatial_argument!, RPar];
        args.push(if try_eat!(self, "->") {
            self.spatial_argument()?
        } else {
            EntityRoutePtr::Root(RootIdentifier::Void).into()
        });
        Ok(args)
    }

    pub(crate) fn angled_generics(&mut self) -> AtomResult<ThinVec<SpatialArgument>> {
        Ok(if try_eat!(self, SpecialToken::LAngle) {
            thin_comma_list![self, spatial_argument!+, ">"]
        } else {
            thin_vec![]
        })
    }

    fn spatial_argument(&mut self) -> AtomResult<SpatialArgument> {
        Ok(if try_eat!(self, "(") {
            let mut args = thin_comma_list!(self, spatial_argument!, ")");
            let scope = if try_eat!(self, "->") {
                args.push(self.spatial_argument()?);
                EntityRoute::default_func_type(args)
            } else {
                EntityRoute::tuple_or_void(args)
            };
            self.intern(scope).into()
        } else {
            get!(self, ty?).into()
        })
    }

    fn intern(&self, scope: EntityRoute) -> EntityRoutePtr {
        self.atom_context
            .entity_syntax_db()
            .intern_entity_route(scope)
    }
}

pub struct AngledSpatialArguments;
impl AtomParserPattern for AngledSpatialArguments {
    type Output = ThinVec<SpatialArgument>;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(
            if parser.try_eat(&BeSpecialTokenPattern(SpecialToken::LAngle))? {
                ThinCommaListPattern {
                    item: SpatialArgumentPattern,
                    terminator: BeSpecialTokenPattern(SpecialToken::RAngle),
                }
                .get_parsed(parser)?
            } else {
                None
            },
        )
    }
}

pub struct SpatialArgumentPattern;

impl AtomParserPattern for SpatialArgumentPattern {
    type Output = SpatialArgument;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        todo!()
    }
}
