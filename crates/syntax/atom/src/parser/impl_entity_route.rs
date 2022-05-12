use entity_kind::TyKind;
use token::SemanticTokenKind;

use super::symbol::SymbolKind;

use super::*;

/// parse atoms from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a> AtomLRParser<'a> {
    pub(crate) fn symbol(&mut self) -> AtomResult<Option<AtomVariant>> {
        Ok(if let Some(token) = self.stream.next() {
            if token.kind == Special::LBox.into() {
                self.push_abs_semantic_token(AbsSemanticToken::new(
                    SemanticTokenKind::Special,
                    token.range,
                ));
                Some(AtomVariant::EntityRoute {
                    route: self.symbolic_ty()?,
                    kind: EntityKind::Type(TyKind::Vec),
                })
            } else if let TokenKind::Identifier(ident) = token.kind {
                let symbol_kind = self
                    .symbol_context
                    .resolve_symbol_kind(ident, token.range)?;
                Some(match symbol_kind {
                    SymbolKind::EntityRoute(route) => {
                        self.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::Entity(self.symbol_context.entity_kind(route)),
                            token.range,
                        ));
                        self.normal_route(route)?
                    }
                    SymbolKind::Variable { init_row } => {
                        self.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::Variable,
                            token.range,
                        ));
                        match ident {
                            Identifier::Builtin(_) | Identifier::Contextual(_) => panic!(),
                            Identifier::Custom(varname) => {
                                AtomVariant::Variable { varname, init_row }
                            }
                        }
                    }
                    SymbolKind::Unrecognized(ident) => AtomVariant::Unrecognized(ident),
                    SymbolKind::ThisData {
                        opt_ty,
                        opt_contract,
                    } => {
                        self.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::ThisData,
                            token.range,
                        ));
                        AtomVariant::ThisData {
                            opt_ty,
                            opt_contract,
                        }
                    }
                    SymbolKind::FrameVariable { init_row } => {
                        self.push_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::FrameVariable,
                            token.range,
                        ));
                        AtomVariant::FrameVariable {
                            varname: ident.custom(),
                            init_row,
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
        Ok(self
            .symbol_context
            .db
            .intern_entity_route(if next_matches!(self, Special::RBox) {
                self.vec_ty()
            } else {
                self.array_ty()
            }?))
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
        let mut route = self
            .symbol_context
            .db
            .make_route(route, self.generics(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ranged_ident = get!(self, custom_ident);
            route = self.symbol_context.db.make_subroute(
                route,
                ranged_ident.ident,
                self.generics(route)?,
            );
            self.push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Entity(self.symbol_context.entity_kind(route)),
                ranged_ident.range,
            ));
            route = self
                .symbol_context
                .db
                .make_route(route, self.generics(route)?);
        }
        return Ok(AtomVariant::EntityRoute {
            route,
            kind: self.symbol_context.entity_kind(route),
        });
    }

    pub(crate) fn ty(&mut self) -> AtomResult<Option<EntityRoutePtr>> {
        Ok(
            if let Some(AtomVariant::EntityRoute {
                route: scope, kind, ..
            }) = self.symbol()?
            {
                if let EntityKind::Type(_) = kind {
                    Some(scope)
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
                RootIdentifier::Type => todo!(),
            },
            _ => match self.symbol_context.entity_kind(route) {
                EntityKind::Module
                | EntityKind::EnumLiteral
                | EntityKind::Feature
                | EntityKind::Member(_) => Ok(Vec::new()),
                EntityKind::Type(_)
                | EntityKind::Trait
                | EntityKind::Routine
                | EntityKind::Pattern => self.angled_generics(),
                EntityKind::Main => panic!(),
            },
        }
    }

    fn func_args(&mut self) -> AtomResult<Vec<GenericArgument>> {
        if !next_matches!(self, "(") {
            return err!("args", self.stream.pop_range());
        }
        let mut args = comma_list![self, generic!, RPar];
        args.push(if next_matches!(self, "->") {
            self.generic()?
        } else {
            EntityRoutePtr::Root(RootIdentifier::Void).into()
        });
        Ok(args)
    }

    fn angled_generics(&mut self) -> AtomResult<Vec<GenericArgument>> {
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
        self.symbol_context.db.intern_entity_route(scope)
    }
}
