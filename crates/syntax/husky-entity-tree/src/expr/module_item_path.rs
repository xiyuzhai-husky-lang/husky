use crate::*;
use husky_token::*;
use original_error::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
/// only module item path is allowed
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ModuleItemPathExpr {
    Root {
        name_token: PathNameToken,
        entity_path: EntityPath,
    },
    Subentity {
        name_token: PathNameToken,
        scope_resolution_token: ScopeResolutionToken,
        subexpr: ModuleItemPathExprIdx,
    },
}
pub type MajorPathExprArena = Arena<ModuleItemPathExpr>;
pub type ModuleItemPathExprIdx = ArenaIdx<ModuleItemPathExpr>;
pub type MajorPathExprIdxRange = ArenaIdxRange<ModuleItemPathExpr>;

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
    ExpectedName(TokenStreamState),
    #[error("NoSuchSubentity")]
    NoSuchSubentity,
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

pub type ModuleItemPathExprResult<T> = Result<T, MajorPathExprError>;

pub(crate) struct ModuleItemPathExprParser<'a, 'b> {
    db: &'b dyn EntityTreeDb,
    crate_root_path: ModulePath,
    token_stream: TokenStream<'a>,
    major_path_expr_arena: &'b mut MajorPathExprArena,
    entity_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
}

impl<'a, 'b> ModuleItemPathExprParser<'a, 'b> {
    pub(crate) fn new(
        db: &'b dyn EntityTreeDb,
        crate_root_path: ModulePath,
        token_stream: TokenStream<'a>,
        major_path_expr_arena: &'b mut MajorPathExprArena,
        entity_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
    ) -> Self {
        Self {
            db,
            crate_root_path,
            token_stream,
            major_path_expr_arena,
            entity_tree_symbol_context,
        }
    }
}

impl<'a, 'b> ModuleItemPathExprParser<'a, 'b> {
    pub(crate) fn parse_major_path_expr(
        &mut self,
    ) -> ModuleItemPathExprResult<(ModuleItemPathExprIdx, ModuleItemPath)> {
        let name_token: PathNameToken =
            self.parse_expected(OriginalMajorPathExprError::ExpectedName)?;
        let entity_path = match name_token {
            PathNameToken::Ident(ident_token) => self
                .entity_tree_symbol_context
                .resolve_ident(ident_token)
                .ok_or(OriginalMajorPathExprError::UnrecognizedIdent(ident_token))?
                .path(self.db),
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => todo!(),
            PathNameToken::Super(_) => todo!(),
        };
        self.parse_major_path_expr_aux(entity_path, name_token)
    }

    fn parse_major_path_subexpr(
        &mut self,
        parent: ModulePath,
    ) -> ModuleItemPathExprResult<(ModuleItemPathExprIdx, ModuleItemPath)> {
        let name_token: PathNameToken =
            self.parse_expected(OriginalMajorPathExprError::ExpectedName)?;
        let entity_path = match name_token {
            PathNameToken::Ident(ident_token) => self
                .entity_tree_symbol_context
                .resolve_subentity(parent.into(), ident_token.ident())
                .ok_or(OriginalMajorPathExprError::NoSuchSubentity)?
                .path(self.db),
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => todo!(),
            PathNameToken::Super(_) => todo!(),
        };
        self.parse_major_path_expr_aux(entity_path, name_token)
    }

    fn parse_major_path_expr_aux(
        &mut self,
        entity_path: EntityPath,
        name_token: PathNameToken,
    ) -> ModuleItemPathExprResult<(ArenaIdx<ModuleItemPathExpr>, ModuleItemPath)> {
        let (expr, module_item_path) = if let Some(scope_resolution_token) =
            self.parse_err_as_none::<ScopeResolutionToken>()
        {
            match entity_path {
                EntityPath::Module(parent) => {
                    let (subexpr, module_item_path) = self.parse_major_path_subexpr(parent)?;
                    (
                        ModuleItemPathExpr::Subentity {
                            name_token,
                            scope_resolution_token,
                            subexpr,
                        },
                        module_item_path,
                    )
                }
                EntityPath::ModuleItem(_) => todo!(),
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::TypeVariant(_) => todo!(),
            }
        } else {
            let EntityPath::ModuleItem(module_item_path) = entity_path else {
                todo!()
            };
            (
                ModuleItemPathExpr::Root {
                    name_token,
                    entity_path,
                },
                module_item_path,
            )
        };
        let expr = self.major_path_expr_arena.alloc_one(expr);
        Ok((expr, module_item_path))
    }
}

impl<'a, 'b> std::ops::Deref for ModuleItemPathExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for ModuleItemPathExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for ModuleItemPathExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for ModuleItemPathExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for ModuleItemPathExprParser<'a, 'b> {}
