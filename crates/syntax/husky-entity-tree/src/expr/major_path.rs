use crate::*;
use husky_token::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
/// only module item path is allowed
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum MajorPathExpr {
    Root {
        ident_token: IdentToken,
        entity_path: EntityPath,
    },
    Subentity {
        parent: MajorPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: IdentToken,
    },
}
pub type MajorPathExprArena = Arena<MajorPathExpr>;
pub type MajorPathExprIdx = ArenaIdx<MajorPathExpr>;
pub type MajorPathExprIdxRange = ArenaIdxRange<MajorPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum MajorPathExprError {
    #[error("{0}")]
    Original(#[from] OriginalMajorPathExprError),
    #[error("{0}")]
    Derived(#[from] DerivedMajorPathExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum OriginalMajorPathExprError {
    #[error("unrecognized identifier")]
    UnrecognizedIdent(IdentToken),
    #[error("expect identifier")]
    ExpectIdent(TokenIdx),
}

impl OriginalError for OriginalMajorPathExprError {
    type Error = MajorPathExprError;
}

impl From<TokenError> for MajorPathExprError {
    fn from(value: TokenError) -> Self {
        MajorPathExprError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum DerivedMajorPathExprError {
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type MajorPathExprResult<T> = Result<T, MajorPathExprError>;

pub(crate) struct MajorPathExprParser<'a, 'b> {
    db: &'b dyn EntityTreeDb,
    token_stream: TokenStream<'a>,
    major_path_expr_arena: &'b mut MajorPathExprArena,
    module_symbol_context: ModuleSymbolContext<'a>,
}

impl<'a, 'b> MajorPathExprParser<'a, 'b> {
    pub(crate) fn new(
        db: &'b dyn EntityTreeDb,
        token_stream: TokenStream<'a>,
        major_path_expr_arena: &'b mut MajorPathExprArena,
        module_symbol_context: ModuleSymbolContext<'a>,
    ) -> Self {
        Self {
            db,
            token_stream,
            major_path_expr_arena,
            module_symbol_context,
        }
    }
}

impl<'a, 'b> MajorPathExprParser<'a, 'b> {
    pub(crate) fn parse_major_path_expr(
        &mut self,
    ) -> MajorPathExprResult<(MajorPathExprIdx, ModuleItemPath)> {
        let ident_token: IdentToken =
            self.parse_expected(OriginalMajorPathExprError::ExpectIdent)?;
        let Some(entity_symbol) = self
            .module_symbol_context
            .resolve_ident(ident_token.token_idx(),ident_token.ident()) else {
                return Err(OriginalMajorPathExprError::UnrecognizedIdent(ident_token).into())
            };
        let path = match entity_symbol {
            EntitySymbol::CrateRoot { .. } => todo!(),
            EntitySymbol::SelfModule { module_path } => todo!(),
            EntitySymbol::PackageDependency { entity_path } => todo!(),
            EntitySymbol::Submodule(_) => todo!(),
            EntitySymbol::ModuleItem(symbol) => symbol.path(self.db),
            EntitySymbol::Use(symbol) => match symbol.path(self.db) {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => path,
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::TypeVariant(_) => todo!(),
            },
            EntitySymbol::SuperModule {
                current_module_path,
                super_module_path,
            } => todo!(),
        };
        let (expr, path) = if let Some(_) = self.try_parse::<ScopeResolutionToken>() {
            todo!()
        } else {
            (
                MajorPathExpr::Root {
                    ident_token,
                    entity_path: path.into(),
                },
                path,
            )
        };
        let expr = self.major_path_expr_arena.alloc_one(expr);
        Ok((expr, path))
    }
}

impl<'a, 'b> std::ops::Deref for MajorPathExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for MajorPathExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for MajorPathExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for MajorPathExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for MajorPathExprParser<'a, 'b> {}
