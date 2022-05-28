use entity_kind::TyKind;
use entity_route::RangedEntityRoute;
use token::SemanticTokenKind;

use super::context::SymbolKind;

use super::*;

/// parse atoms from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn symbol(&mut self) -> AtomResult<Option<AtomVariant>> {
        Ok(if let Some(token) = self.token_stream.next() {
            if token.kind == Special::LBox.into() {
                self.context.push_abs_semantic_token(AbsSemanticToken::new(
                    SemanticTokenKind::Special,
                    token.range,
                ));
                Some(AtomVariant::EntityRoute {
                    route: self.symbolic_ty()?,
                    kind: EntityKind::Type(TyKind::Vec),
                })
            } else if let TokenKind::Identifier(ident) = token.kind {
                let symbol_kind = self.context.resolve_symbol_kind(ident, token.range)?;
                Some(match symbol_kind {
                    SymbolKind::EntityRoute(route) => {
                        self.context.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::Entity(
                                self.context.entity_kind(route, token.range)?,
                            ),
                            token.range,
                        ));
                        self.normal_route(route)?
                    }
                    SymbolKind::Variable { init_range } => {
                        self.context.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::Variable,
                            token.range,
                        ));
                        match ident {
                            Identifier::Builtin(_) | Identifier::Contextual(_) => panic!(),
                            Identifier::Custom(varname) => AtomVariant::Variable {
                                varname,
                                init_range,
                            },
                        }
                    }
                    SymbolKind::Unrecognized(ident) => AtomVariant::Unrecognized(ident),
                    SymbolKind::ThisData {
                        opt_ty,
                        opt_contract,
                    } => {
                        self.context.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::ThisData,
                            token.range,
                        ));
                        AtomVariant::ThisData {
                            opt_ty,
                            opt_contract,
                        }
                    }
                    SymbolKind::FrameVariable { init_range } => {
                        self.context.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::FrameVariable,
                            token.range,
                        ));
                        AtomVariant::FrameVariable {
                            varname: ident.custom(),
                            init_range,
                        }
                    }
                })
            } else {
                None
            }
        } else {
            None
        })
    }

    fn symbolic_ty(&mut self) -> AtomResult<EntityRoutePtr> {
        let route = if next_matches!(self, Special::RBox) {
            self.vec_ty()
        } else {
            self.array_ty()
        }?;
        Ok(self.context.entity_syntax_db().intern_entity_route(route))
    }

    fn vec_ty(&mut self) -> AtomResult<EntityRoute> {
        Ok(EntityRoute::vec(self.generic()?))
    }

    fn array_ty(&mut self) -> AtomResult<EntityRoute> {
        let size = get!(self, usize_literal);
        no_look_pass!(self, special, Special::RBox);
        let element = self.generic()?;
        Ok(EntityRoute::array(element, size))
    }

    fn normal_route(&mut self, route: EntityRoutePtr) -> AtomResult<AtomVariant> {
        let generic_arguments = self.generics(route)?;
        let mut route = self
            .context
            .entity_syntax_db()
            .make_route(route, generic_arguments);
        while next_matches!(self, Special::DoubleColon) {
            let ranged_ident = get!(self, custom_ident);
            let generics = self.generics(route)?;
            route =
                self.context
                    .entity_syntax_db()
                    .make_subroute(route, ranged_ident.ident, generics);
            self.context.push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Entity(self.context.entity_kind(route, ranged_ident.range)?),
                ranged_ident.range,
            ));
            let generic_arguments = self.generics(route)?;
            route = self
                .context
                .entity_syntax_db()
                .make_route(route, generic_arguments);
        }
        return Ok(AtomVariant::EntityRoute {
            route,
            kind: self.context.entity_kind(route, Default::default()).unwrap(),
        });
    }

    pub(crate) fn ty(&mut self) -> AtomResult<Option<EntityRoutePtr>> {
        Ok(
            if let Some(AtomVariant::EntityRoute { route, kind, .. }) = self.symbol()? {
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

    fn generics(&mut self, route: EntityRoutePtr) -> AtomResult<Vec<GenericArgument>> {
        if route.generic_arguments.len() > 0 {
            todo!()
        }
        match route.kind {
            EntityRouteKind::Root { ident } => match ident {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::F32
                | RootIdentifier::B32
                | RootIdentifier::B64
                | RootIdentifier::Bool
                | RootIdentifier::True
                | RootIdentifier::False
                | RootIdentifier::Debug
                | RootIdentifier::Std
                | RootIdentifier::Core
                | RootIdentifier::Datasets
                | RootIdentifier::CloneTrait
                | RootIdentifier::CopyTrait
                | RootIdentifier::PartialEqTrait
                | RootIdentifier::EqTrait => Ok(Vec::new()),
                RootIdentifier::Fp
                | RootIdentifier::Fn
                | RootIdentifier::FnMut
                | RootIdentifier::FnOnce => Ok(self.func_args()?),
                RootIdentifier::Vec
                | RootIdentifier::Array
                | RootIdentifier::Tuple
                | RootIdentifier::DatasetType => self.angled_generics(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::ModuleType => todo!(),
            },
            _ => match self.context.entity_kind(route, Default::default()).unwrap() {
                EntityKind::Module
                | EntityKind::EnumLiteral
                | EntityKind::Feature
                | EntityKind::Member(_) => Ok(Vec::new()),
                EntityKind::Type(_) | EntityKind::Trait | EntityKind::Function { .. } => {
                    self.angled_generics()
                }
                EntityKind::Main => panic!(),
            },
        }
    }

    fn func_args(&mut self) -> AtomResult<Vec<GenericArgument>> {
        if !next_matches!(self, "(") {
            return err!("args", self.token_stream.pop_text_range());
        }
        let mut args = comma_list![self, generic!, RPar];
        args.push(if next_matches!(self, "->") {
            self.generic()?
        } else {
            EntityRoutePtr::Root(RootIdentifier::Void).into()
        });
        Ok(args)
    }

    pub(crate) fn angled_generics(&mut self) -> AtomResult<Vec<GenericArgument>> {
        Ok(if next_matches!(self, Special::LAngle) {
            comma_list![self, generic!+, ">"]
        } else {
            Vec::new()
        })
    }

    fn generic(&mut self) -> AtomResult<GenericArgument> {
        Ok(if next_matches!(self, "(") {
            let mut args = comma_list!(self, generic!, ")");
            let scope = if next_matches!(self, "->") {
                args.push(self.generic()?);
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
        self.context.entity_syntax_db().intern_entity_route(scope)
    }
}
