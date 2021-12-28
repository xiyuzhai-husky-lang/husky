use scope::LifetimeParameter;

use super::*;

/// parse scope from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a> ScopeLRParser<'a> {
    pub(crate) fn scope(&mut self) -> Result<Option<(ScopeId, ScopeKind)>, AtomError> {
        Ok(if let Some(token) = self.stream.next() {
            if token.kind == Special::LBox.into() {
                Some((self.symbolic_ty()?, ScopeKind::Type))
            } else if let Some(route) = self.scope_proxy.resolve_scope_route(token) {
                Some(self.normal_scope(route)?)
            } else {
                None
            }
        } else {
            None
        })
    }

    fn symbolic_ty(&mut self) -> AtomResult<ScopeId> {
        Ok(self
            .scope_proxy
            .db
            .intern_scope(if next_matches!(self, Special::RBox) {
                self.vec_ty()
            } else {
                self.array_ty()
            }?))
    }

    fn vec_ty(&mut self) -> AtomResult<Scope> {
        Ok(Scope::vec(self.generic()?))
    }

    fn array_ty(&mut self) -> AtomResult<Scope> {
        let size = get!(self, usize_literal);
        no_look_pass!(self, special, Special::RBox);
        let element = self.generic()?;
        Ok(Scope::array(element, size))
    }

    fn normal_scope(&mut self, route: ScopeRoute) -> AtomResult<(ScopeId, ScopeKind)> {
        let mut scope = self
            .scope_proxy
            .db
            .make_scope(route, self.lifetimes_and_generics(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ident = get!(self, custom_ident);
            scope = self.scope_proxy.db.make_child_scope(
                scope,
                ident,
                self.lifetimes_and_generics(route)?,
            );
        }
        return Ok((scope, self.scope_proxy.db.scope_kind(scope)));
    }

    pub(crate) fn ty(&mut self) -> AtomResult<Option<ScopeId>> {
        Ok(if let Some((scope, kind)) = self.scope()? {
            if kind == ScopeKind::Type {
                Some(scope)
            } else {
                None
            }
        } else {
            None
        })
    }

    fn lifetimes_and_generics(
        &mut self,
        route: ScopeRoute,
    ) -> AtomResult<(Vec<LifetimeParameter>, Vec<GenericArgument>)> {
        match route {
            ScopeRoute::Builtin(ident) => match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::Debug
                | BuiltinIdentifier::Std
                | BuiltinIdentifier::Core => Ok((Vec::new(), Vec::new())),
                BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce => Ok((Vec::new(), self.func_args()?)),
                BuiltinIdentifier::Vector | BuiltinIdentifier::Array | BuiltinIdentifier::Tuple => {
                    self.angled_lifetimes_and_generics()
                }
            },
            _ => match self.scope_proxy.db.scope_kind_from_route(route) {
                ScopeKind::Module | ScopeKind::Value => Ok((Vec::new(), Vec::new())),
                ScopeKind::Type | ScopeKind::Trait | ScopeKind::Func => {
                    self.angled_lifetimes_and_generics()
                }
            },
        }
    }

    fn func_args(&mut self) -> Result<Vec<GenericArgument>, AtomError> {
        if !next_matches!(self, "(") {
            return atom_err!(self.stream.pop_range(), "args");
        }
        let mut args = comma_list![self, generic!, RPar];
        args.push(if next_matches!(self, "->") {
            self.generic()?
        } else {
            ScopeId::Builtin(BuiltinIdentifier::Void).into()
        });
        Ok(args)
    }

    fn angled_lifetimes_and_generics(
        &mut self,
    ) -> Result<(Vec<LifetimeParameter>, Vec<GenericArgument>), AtomError> {
        Ok(if next_matches!(self, Special::LAngle) {
            comma_list![self, lifetime?, generic!, ">"]
        } else {
            (Vec::new(), Vec::new())
        })
    }

    fn lifetime(&mut self) -> AtomResult<Option<LifetimeParameter>> {
        if next_matches!(self, "'") {
            if next_matches!(self, "_") {
                Ok(Some(LifetimeParameter::Elided))
            } else {
                let ident = get!(self, custom_ident);
                Ok(Some(LifetimeParameter::Explicit(ident)))
            }
        } else {
            Ok(None)
        }
    }

    fn generic(&mut self) -> AtomResult<GenericArgument> {
        Ok(if next_matches!(self, "(") {
            let mut args = comma_list!(self, generic!, ")");
            let scope = if next_matches!(self, "->") {
                args.push(self.generic()?);
                Scope::default_func_type(args)
            } else {
                Scope::tuple_or_void(args)
            };
            self.intern(scope).into()
        } else {
            get!(self, ty?).into()
        })
    }

    fn intern(&self, scope: Scope) -> ScopeId {
        self.scope_proxy.db.intern_scope(scope)
    }
}
