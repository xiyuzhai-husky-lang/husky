use super::symbol_proxy::SymbolKind;

use super::*;

/// parse scope from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a> AtomLRParser<'a> {
    pub(crate) fn symbol(&mut self) -> AstResult<Option<AtomKind>> {
        Ok(if let Some(token) = self.stream.next() {
            if token.kind == Special::LBox.into() {
                Some(AtomKind::Scope(self.symbolic_ty()?, ScopeKind::Type))
            } else if let TokenKind::Identifier(ident) = token.kind {
                let symbol_kind = self.scope_proxy.resolve_symbol_kind(ident, &token.range)?;
                match symbol_kind {
                    SymbolKind::Scope(route) => Some(self.normal_scope(route)?),
                    SymbolKind::Variable(_) => match ident {
                        Identifier::Builtin(_) | Identifier::Implicit(_) => panic!(),
                        Identifier::Custom(ident) => Some(AtomKind::Variable(ident)),
                    },
                }
            } else {
                None
            }
        } else {
            None
        })
    }

    fn symbolic_ty(&mut self) -> AstResult<ScopePtr> {
        Ok(self
            .scope_proxy
            .db
            .intern_scope(if next_matches!(self, Special::RBox) {
                self.vec_ty()
            } else {
                self.array_ty()
            }?))
    }

    fn vec_ty(&mut self) -> AstResult<Scope> {
        Ok(Scope::vec(self.generic()?))
    }

    fn array_ty(&mut self) -> AstResult<Scope> {
        let size = get!(self, usize_literal);
        no_look_pass!(self, special, Special::RBox);
        let element = self.generic()?;
        Ok(Scope::array(element, size))
    }

    fn normal_scope(&mut self, route: ScopeRoute) -> AstResult<AtomKind> {
        let mut scope = self.scope_proxy.db.make_scope(route, self.generics(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ident = get!(self, custom_ident);
            scope = self
                .scope_proxy
                .db
                .make_child_scope(scope, ident, self.generics(route)?);
        }
        return Ok(AtomKind::Scope(
            scope,
            self.scope_proxy.db.scope_kind(scope),
        ));
    }

    pub(crate) fn ty(&mut self) -> AstResult<Option<ScopePtr>> {
        Ok(if let Some(AtomKind::Scope(scope, kind)) = self.symbol()? {
            if kind == ScopeKind::Type {
                Some(scope)
            } else {
                None
            }
        } else {
            None
        })
    }

    fn generics(&mut self, route: ScopeRoute) -> AstResult<Vec<GenericArgument>> {
        match route {
            ScopeRoute::Builtin { ident } => match ident {
                BuiltinIdentifier::Void
                | BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::B32
                | BuiltinIdentifier::B64
                | BuiltinIdentifier::Bool
                | BuiltinIdentifier::True
                | BuiltinIdentifier::False
                | BuiltinIdentifier::Debug
                | BuiltinIdentifier::Std
                | BuiltinIdentifier::Core => Ok(Vec::new()),
                BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce => Ok(self.func_args()?),
                BuiltinIdentifier::Vector
                | BuiltinIdentifier::Array
                | BuiltinIdentifier::Tuple
                | BuiltinIdentifier::DatasetType => self.angled_generics(),
            },
            _ => match self.scope_proxy.db.scope_kind_from_route(route) {
                ScopeKind::Module | ScopeKind::Literal | ScopeKind::Feature => Ok(Vec::new()),
                ScopeKind::Type | ScopeKind::Trait | ScopeKind::Func => self.angled_generics(),
            },
        }
    }

    fn func_args(&mut self) -> AstResult<Vec<GenericArgument>> {
        if !next_matches!(self, "(") {
            return ast_err!(self.stream.pop_range(), "args");
        }
        let mut args = comma_list![self, generic!, RPar];
        args.push(if next_matches!(self, "->") {
            self.generic()?
        } else {
            ScopePtr::Builtin(BuiltinIdentifier::Void).into()
        });
        Ok(args)
    }

    fn angled_generics(&mut self) -> AstResult<Vec<GenericArgument>> {
        Ok(if next_matches!(self, Special::LAngle) {
            comma_list![self, generic!+, ">"]
        } else {
            Vec::new()
        })
    }

    fn generic(&mut self) -> AstResult<GenericArgument> {
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

    fn intern(&self, scope: Scope) -> ScopePtr {
        self.scope_proxy.db.intern_scope(scope)
    }
}
