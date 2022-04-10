use entity_syntax::TyKind;

use super::symbol_proxy::SymbolKind;

use super::*;

/// parse scope from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a> AtomLRParser<'a> {
    pub(crate) fn symbol(&mut self) -> AtomResult<Option<AtomKind>> {
        Ok(if let Some(token) = self.stream.next() {
            if token.kind == Special::LBox.into() {
                Some(AtomKind::EntityRoute {
                    route: self.symbolic_ty()?,
                    kind: EntityKind::Type(TyKind::Vec),
                })
            } else if let TokenKind::Identifier(ident) = token.kind {
                let symbol_kind = self.scope_proxy.resolve_symbol_kind(ident, token.range)?;
                Some(match symbol_kind {
                    SymbolKind::EntityRoute(route) => self.normal_scope(route)?,
                    SymbolKind::Variable { init_row } => match ident {
                        Identifier::Builtin(_) | Identifier::Contextual(_) => panic!(),
                        Identifier::Custom(varname) => AtomKind::Variable { varname, init_row },
                    },
                    SymbolKind::Unrecognized(ident) => AtomKind::Unrecognized(ident),
                    SymbolKind::ThisData { ty } => AtomKind::ThisData { ty },
                    SymbolKind::ThisType { ty } => AtomKind::ThisType { ty },
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
            .scope_proxy
            .db
            .intern_scope(if next_matches!(self, Special::RBox) {
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

    fn normal_scope(&mut self, route: EntityRoutePtr) -> AtomResult<AtomKind> {
        let mut scope = self.scope_proxy.db.make_scope(route, self.generics(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ident = get!(self, custom_ident);
            scope = self
                .scope_proxy
                .db
                .make_child_scope(scope, ident, self.generics(route)?);
        }
        return Ok(AtomKind::EntityRoute {
            route: scope,
            kind: self.scope_proxy.db.raw_entity_kind(scope),
        });
    }

    pub(crate) fn ty(&mut self) -> AtomResult<Option<EntityRoutePtr>> {
        Ok(
            if let Some(AtomKind::EntityRoute {
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
        if route.generics.len() > 0 {
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
            _ => match self
                .scope_proxy
                .db
                .raw_entity_kind_from_scope_kind(route.kind)
            {
                EntityKind::Module | EntityKind::Literal | EntityKind::Feature => Ok(Vec::new()),
                EntityKind::Type(_)
                | EntityKind::Trait
                | EntityKind::Routine
                | EntityKind::Pattern => self.angled_generics(),
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
        self.scope_proxy.db.intern_scope(scope)
    }
}
