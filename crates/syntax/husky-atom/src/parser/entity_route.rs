use super::context::SymbolKind;
use super::*;
use husky_entity_kind::TyKind;
use husky_entity_route::RangedEntityRoute;
use husky_text::RangedCustomIdentifier;
use husky_token::{is_special, special_token, SemanticTokenKind};
use thin_vec::{thin_vec, ThinVec};

/// parse atoms from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    pub(crate) fn symbol(&mut self) -> AtomResult<Option<HuskyAtomVariant>> {
        Ok(if let Some(token) = self.token_stream.next() {
            if is_special!(token, "[") {
                self.atom_context
                    .push_abs_semantic_token(AbsSemanticToken::new(
                        SemanticTokenKind::Special,
                        token.range,
                    ));
                let (route, ty_kind) = if try_eat_special!(self, "]") {
                    if let Some(element_ty) = deprecated_try_get!(self, ty?) {
                        (EntityRoute::vec(element_ty), TyKind::Vec)
                    } else {
                        return Ok(None);
                    }
                } else if try_eat_special!(self, "%") {
                    eat_special!(self, "]");
                    let element = self.spatial_argument()?;
                    (
                        self.db()
                            .entity_route_menu()
                            .std_slice_cyclic_slice
                            .call([element]),
                        TyKind::CyclicSlice,
                    )
                } else if let Some(size) = self.try_get(&UsizeLiteralPattern)? {
                    if !try_eat_special!(self, "]") {
                        return Ok(None);
                    }
                    if let Some(token) = self.token_stream.peek() {
                        match token.left_convexity() {
                            Some(Convexity::Convex) => (
                                EntityRoute::array(deprecated_get!(self, ty?), size),
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
                let ty = deprecated_get!(self, ty?);
                Some(HuskyAtomVariant::EntityRoute {
                    route: self
                        .db()
                        .route_call(RootBuiltinIdentifier::Ref.into(), thin_vec![ty.into()]),
                    kind: EntityKind::Type(TyKind::Ref),
                })
            } else if is_special!(token, "?") {
                let ty = deprecated_get!(self, ty?);
                Some(HuskyAtomVariant::EntityRoute {
                    route: self
                        .db()
                        .route_call(RootBuiltinIdentifier::Option.into(), thin_vec![ty.into()]),
                    kind: EntityKind::Type(TyKind::Option),
                })
            } else if let TokenKind::Identifier(ident) = token.kind {
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
                            Identifier::Root(_) | Identifier::Contextual(_) => panic!(),
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

    fn normal_route(&mut self, route: EntityRouteItd) -> AtomResult<HuskyAtomVariant> {
        let generic_arguments = self.generics(route)?;
        let mut route = self
            .atom_context
            .entity_syntax_db()
            .route_call(route, generic_arguments);
        while deprecated_try_eat!(self, SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)) {
            let ranged_ident = deprecated_get!(self, custom_ident);
            let new_route = self
                .db()
                .subroute(route, ranged_ident.ident, Default::default());
            match self
                .atom_context
                .entity_syntax_db()
                .husky_entity_kind(new_route)
            {
                Ok(_) => (),
                Err(e) => {
                    let message = e.message;
                    err!(format!("{message}"), ranged_ident.range)?
                }
            }
            let generic_arguments = self.generics(new_route)?;
            route = self
                .db()
                .subroute(route, ranged_ident.ident, generic_arguments);
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

    pub(crate) fn ty(&mut self) -> AtomResult<Option<EntityRouteItd>> {
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

    fn generics(&mut self, route: EntityRouteItd) -> AtomResult<ThinVec<SpatialArgument>> {
        if route.spatial_arguments.len() > 0 {
            p!(route);
            todo!()
        }
        match route.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootBuiltinIdentifier::Void
                | RootBuiltinIdentifier::I32
                | RootBuiltinIdentifier::I64
                | RootBuiltinIdentifier::F32
                | RootBuiltinIdentifier::F64
                | RootBuiltinIdentifier::B32
                | RootBuiltinIdentifier::B64
                | RootBuiltinIdentifier::Bool
                | RootBuiltinIdentifier::True
                | RootBuiltinIdentifier::False
                | RootBuiltinIdentifier::Debug
                | RootBuiltinIdentifier::Std
                | RootBuiltinIdentifier::Core
                | RootBuiltinIdentifier::Domains
                | RootBuiltinIdentifier::CloneTrait
                | RootBuiltinIdentifier::CopyTrait
                | RootBuiltinIdentifier::PartialEqTrait
                | RootBuiltinIdentifier::EqTrait
                | RootBuiltinIdentifier::TypeType
                | RootBuiltinIdentifier::Trait
                | RootBuiltinIdentifier::Module => Ok(thin_vec![]),
                RootBuiltinIdentifier::Mor
                | RootBuiltinIdentifier::ThickFp
                | RootBuiltinIdentifier::Fn
                | RootBuiltinIdentifier::FnMut
                | RootBuiltinIdentifier::FnOnce => Ok(self.func_args()?),
                RootBuiltinIdentifier::Vec
                | RootBuiltinIdentifier::Array
                | RootBuiltinIdentifier::Tuple
                | RootBuiltinIdentifier::DatasetType
                | RootBuiltinIdentifier::Ref
                | RootBuiltinIdentifier::Option => Ok(self.angled_generics()?.expect("todo")),
                RootBuiltinIdentifier::VisualType => todo!(),
                RootBuiltinIdentifier::RefMut => todo!(),
            },
            _ => {
                let husky_entity_kind = self
                    .atom_context
                    .entity_kind(route, Default::default())
                    .unwrap();
                match husky_entity_kind {
                    EntityKind::Module
                    | EntityKind::EnumVariant
                    | EntityKind::Feature
                    | EntityKind::Member(_) => Ok(thin_vec![]),
                    EntityKind::Type(_) | EntityKind::Trait | EntityKind::Function { .. } => {
                        Ok(self.angled_generics()?.expect("todo"))
                    }
                    EntityKind::Main => panic!(),
                }
            }
        }
    }

    fn func_args(&mut self) -> AtomResult<ThinVec<SpatialArgument>> {
        if !try_eat_special!(self, "(") {
            return Ok(self.angled_generics()?.expect("todo"));
        }
        let mut args =
            deprecated_thin_comma_list![self, spatial_argument!, SpecialToken::Ket(Bracket::Par)];
        args.push(if deprecated_try_eat!(self, "->") {
            self.spatial_argument()?
        } else {
            EntityRouteItd::Root(RootBuiltinIdentifier::Void).into()
        });
        Ok(args)
    }

    pub(crate) fn angled_generics(&mut self) -> AtomResult<Option<ThinVec<SpatialArgument>>> {
        Ok(if try_eat_special!(self, "<") {
            self.try_get(&ThinCommaListPattern {
                item: SpatialArgumentPattern,
                terminator: be_special_token_patt!(">"),
            })?
        } else {
            Some(Default::default())
        })
    }

    fn spatial_argument(&mut self) -> AtomResult<SpatialArgument> {
        Ok(if deprecated_try_eat!(self, "(") {
            let mut args = deprecated_thin_comma_list!(self, spatial_argument!, ")");
            let scope = if deprecated_try_eat!(self, "->") {
                args.push(self.spatial_argument()?);
                EntityRoute::default_func_type(args)
            } else {
                EntityRoute::tuple_or_void(args)
            };
            self.intern(scope).into()
        } else {
            deprecated_get!(self, ty?).into()
        })
    }

    fn intern(&self, scope: EntityRoute) -> EntityRouteItd {
        self.atom_context
            .entity_syntax_db()
            .intern_entity_route(scope)
    }
}

pub struct AngledSpatialArguments;
impl std::fmt::Display for AngledSpatialArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "angled spatial arguments".fmt(f)
    }
}
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
impl std::fmt::Display for SpatialArgumentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "spatial argument".fmt(f)
    }
}
impl AtomParserPattern for SpatialArgumentPattern {
    type Output = SpatialArgument;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(if try_eat_special!(parser, "(") {
            let mut args = deprecated_thin_comma_list!(parser, spatial_argument!, ")");
            let scope = if try_eat_special!(parser, "->") {
                args.push(parser.spatial_argument()?);
                EntityRoute::default_func_type(args)
            } else {
                EntityRoute::tuple_or_void(args)
            };
            Some(parser.intern(scope).into())
        } else {
            if let Some(ty) = deprecated_try_get!(parser, ty?) {
                Some(ty.into())
            } else {
                None
            }
        })
    }
}
