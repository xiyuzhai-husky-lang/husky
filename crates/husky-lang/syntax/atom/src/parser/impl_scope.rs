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
        Ok(Scope::vec(self.arg()?))
    }

    fn array_ty(&mut self) -> AtomResult<Scope> {
        let size = get!(self, usize_literal);
        no_look_pass!(self, special, Special::RBox);
        let element = self.arg()?;
        Ok(Scope::builtin(
            BuiltinIdentifier::Array,
            vec![element, size.into()],
        ))
    }

    fn normal_scope(&mut self, route: ScopeRoute) -> AtomResult<(ScopeId, ScopeKind)> {
        let mut scope = self.scope_proxy.db.make_scope(route, self.args(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ident = get!(self, custom_ident);
            scope = self
                .scope_proxy
                .db
                .make_child_scope(scope, ident, self.args(route)?);
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

    fn args(&mut self, route: ScopeRoute) -> AtomResult<Vec<GenericArgument>> {
        match route {
            ScopeRoute::Builtin(ident) => match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::Debug
                | BuiltinIdentifier::Std
                | BuiltinIdentifier::Core => Ok(Vec::new()),
                BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce => self.func_args(),
                BuiltinIdentifier::Vector | BuiltinIdentifier::Array | BuiltinIdentifier::Tuple => {
                    self.angled_args()
                }
            },
            _ => match self.scope_proxy.db.scope_kind_from_route(route) {
                ScopeKind::Module | ScopeKind::Value => Ok(Vec::new()),
                ScopeKind::Type | ScopeKind::Trait | ScopeKind::Func => self.angled_args(),
            },
        }
    }

    fn func_args(&mut self) -> Result<Vec<GenericArgument>, AtomError> {
        if !next_matches!(self, "(") {
            return atom_err!(self.stream.pop_range(), "args");
        }
        let mut args = comma_list![self, arg!, RPar];
        args.push(if next_matches!(self, "->") {
            self.arg()?
        } else {
            ScopeId::Builtin(BuiltinIdentifier::Void).into()
        });
        Ok(args)
    }

    fn angled_args(&mut self) -> Result<Vec<GenericArgument>, AtomError> {
        Ok(if next_matches!(self, Special::LAngle) {
            comma_list![self, arg!+, ">"]
        } else {
            Vec::new()
        })
    }

    fn arg(&mut self) -> AtomResult<GenericArgument> {
        Ok(if next_matches!(self, "(") {
            let mut args = comma_list!(self, arg!, ")");
            let scope = if next_matches!(self, "->") {
                args.push(self.arg()?);
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
