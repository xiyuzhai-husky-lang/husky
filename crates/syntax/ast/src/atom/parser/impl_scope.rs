use super::symbol_proxy::SymbolKind;

use super::*;

/// parse scope from left to right
/// it's hard to parse a standalone tuple from left to right,
/// so that is leaved for atom group to handle
impl<'a> AtomLRParser<'a> {
    pub(crate) fn symbol(&mut self) -> AstResult<Option<AtomKind>> {
        Ok(if let Some(token) = self.stream.next() {
            if token.kind == Special::LBox.into() {
                Some(AtomKind::Scope {
                    scope: self.symbolic_ty()?,
                    kind: RawEntityKind::Type(RawTyKind::Other),
                })
            } else if let TokenKind::Identifier(ident) = token.kind {
                let symbol_kind =
                    self.scope_proxy
                        .resolve_symbol_kind(ident, self.file, token.range)?;
                Some(match symbol_kind {
                    SymbolKind::Scope(route) => self.normal_scope(route)?,
                    SymbolKind::Variable { init_row } => match ident {
                        Identifier::Builtin(_) | Identifier::Contextual(_) => panic!(),
                        Identifier::Custom(varname) => AtomKind::Variable { varname, init_row },
                    },
                    SymbolKind::Unrecognized(ident) => AtomKind::Unrecognized(ident),
                    SymbolKind::ThisData { ty } => AtomKind::This { ty },
                })
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

    fn normal_scope(&mut self, route: ScopeKind) -> AstResult<AtomKind> {
        let mut scope = self.scope_proxy.db.make_scope(route, self.generics(route)?);
        while next_matches!(self, Special::DoubleColon) {
            let ident = get!(self, custom_ident);
            scope = self
                .scope_proxy
                .db
                .make_child_scope(scope, ident, self.generics(route)?);
        }
        return Ok(AtomKind::Scope {
            scope,
            kind: self.scope_proxy.db.raw_entity_kind(scope),
        });
    }

    pub(crate) fn ty(&mut self) -> AstResult<Option<ScopePtr>> {
        Ok(
            if let Some(AtomKind::Scope { scope, kind, .. }) = self.symbol()? {
                if let RawEntityKind::Type(_) = kind {
                    Some(scope)
                } else {
                    None
                }
            } else {
                None
            },
        )
    }

    fn generics(&mut self, scope_kind: ScopeKind) -> AstResult<Vec<GenericArgument>> {
        match scope_kind {
            ScopeKind::Builtin { ident } => match ident {
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
                | BuiltinIdentifier::Core
                | BuiltinIdentifier::Datasets => Ok(Vec::new()),
                BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce => Ok(self.func_args()?),
                BuiltinIdentifier::Vec
                | BuiltinIdentifier::Array
                | BuiltinIdentifier::Tuple
                | BuiltinIdentifier::DatasetType => self.angled_generics(),
                BuiltinIdentifier::Type => todo!(),
            },
            _ => match self
                .scope_proxy
                .db
                .raw_entity_kind_from_scope_kind(&scope_kind)
            {
                RawEntityKind::Module | RawEntityKind::Literal | RawEntityKind::Feature => {
                    Ok(Vec::new())
                }
                RawEntityKind::Type(_)
                | RawEntityKind::Trait
                | RawEntityKind::Routine
                | RawEntityKind::Pattern => self.angled_generics(),
            },
        }
    }

    fn func_args(&mut self) -> AstResult<Vec<GenericArgument>> {
        if !next_matches!(self, "(") {
            return err!(self.file, self.stream.pop_range(), "args");
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
