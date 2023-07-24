use super::*;
use husky_print_utils::p;
use husky_token::*;
use maybe_result::*;
use original_error::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
/// only module item path is allowed
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum ModuleItemPathExpr {
    Root {
        name_token: PathNameToken,
        major_path: MajorEntityPath,
    },
    Subitem {
        name_token: PathNameToken,
        scope_resolution_token: ScopeResolutionToken,
        subexpr: ModuleItemPathExprIdx,
    },
}
pub type MajorPathExprArena = Arena<ModuleItemPathExpr>;
pub type ModuleItemPathExprIdx = ArenaIdx<ModuleItemPathExpr>;
pub type MajorPathExprIdxRange = ArenaIdxRange<ModuleItemPathExpr>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum MajorPathExprError {
    #[error("{0}")]
    Original(#[from] OriginalMajorPathExprError),
    #[error("{0}")]
    Derived(#[from] DerivedMajorPathExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum OriginalMajorPathExprError {
    #[error("unrecognized identifier")]
    UnrecognizedIdent(IdentToken),
    #[error("expect identifier")]
    ExpectedName(TokenStreamState),
    #[error("NoSuchSubitem")]
    NoSuchSubitem,
}

impl IntoError for OriginalMajorPathExprError {
    type Error = MajorPathExprError;
}

impl From<TokenError> for MajorPathExprError {
    fn from(value: TokenError) -> Self {
        MajorPathExprError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum DerivedMajorPathExprError {
    #[error("token error")]
    Token(#[from] TokenError),
}

pub type ModuleItemPathExprResult<T> = Result<T, MajorPathExprError>;
pub type ModuleItemPathExprMaybeResult<T> = MaybeResult<T, MajorPathExprError>;

pub(crate) struct ModuleItemPathExprParser<'a, 'b> {
    db: &'b dyn EntitySynTreeDb,
    crate_root_path: ModulePath,
    token_stream: TokenStream<'a>,
    major_path_expr_arena: &'b mut MajorPathExprArena,
    item_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
}

impl<'a, 'b> HasTokenDb for ModuleItemPathExprParser<'a, 'b> {
    fn token_db(&self) -> &dyn TokenDb {
        self.db
    }
}

impl<'a, 'b> ModuleItemPathExprParser<'a, 'b> {
    pub(crate) fn new(
        db: &'b dyn EntitySynTreeDb,
        crate_root_path: ModulePath,
        token_stream: TokenStream<'a>,
        major_path_expr_arena: &'b mut MajorPathExprArena,
        item_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
    ) -> Self {
        Self {
            db,
            crate_root_path,
            token_stream,
            major_path_expr_arena,
            item_tree_symbol_context,
        }
    }
}

impl<'a, 'b> ModuleItemPathExprParser<'a, 'b> {
    // todo: rollback
    pub(crate) fn parse_major_path_expr(
        &mut self,
    ) -> ModuleItemPathExprMaybeResult<(ModuleItemPathExprIdx, ModuleItemPath)> {
        let name_token: PathNameToken = self.try_parse_option()??;
        let path = match name_token {
            PathNameToken::Ident(ident_token) => self
                .item_tree_symbol_context
                .resolve_root_ident(ident_token)?
                .path(self.db)
                .major()?,
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => todo!(),
            PathNameToken::Super(_) => todo!(),
        };
        self.parse_major_path_expr_aux(path, name_token).into()
    }

    pub(crate) fn parse_major_path_expr_expected(
        &mut self,
    ) -> ModuleItemPathExprResult<(ModuleItemPathExprIdx, ModuleItemPath)> {
        let name_token: PathNameToken =
            self.try_parse_expected(OriginalMajorPathExprError::ExpectedName)?;
        let path = match name_token {
            PathNameToken::Ident(ident_token) => {
                match self
                    .item_tree_symbol_context
                    .resolve_root_ident(ident_token)
                    .ok_or(OriginalMajorPathExprError::UnrecognizedIdent(ident_token))?
                    .path(self.db)
                    .major()
                {
                    Some(path) => path,
                    None => todo!(),
                }
            }
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => todo!(),
            PathNameToken::Super(_) => todo!(),
        };
        self.parse_major_path_expr_aux(path, name_token)
    }

    fn parse_major_path_subexpr(
        &mut self,
        parent: ModulePath,
    ) -> ModuleItemPathExprResult<(ModuleItemPathExprIdx, ModuleItemPath)> {
        let name_token: PathNameToken =
            self.try_parse_expected(OriginalMajorPathExprError::ExpectedName)?;
        let major_path = match name_token {
            PathNameToken::Ident(ident_token) => match self
                .item_tree_symbol_context
                .resolve_subitem(parent.into(), ident_token.ident())
                .ok_or(OriginalMajorPathExprError::NoSuchSubitem)?
                .path(self.db)
                .major()
            {
                Some(major_path) => major_path,
                None => todo!(),
            },
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => todo!(),
            PathNameToken::Super(_) => todo!(),
        };
        self.parse_major_path_expr_aux(major_path, name_token)
    }

    fn parse_major_path_expr_aux(
        &mut self,
        major_path: MajorEntityPath,
        name_token: PathNameToken,
    ) -> ModuleItemPathExprResult<(ArenaIdx<ModuleItemPathExpr>, ModuleItemPath)> {
        let (expr, module_item_path) = if let Some(scope_resolution_token) =
            self.try_parse_err_as_none::<ScopeResolutionToken>()
        {
            match major_path {
                MajorEntityPath::Module(parent) => {
                    let (subexpr, module_item_path) = self.parse_major_path_subexpr(parent)?;
                    (
                        ModuleItemPathExpr::Subitem {
                            name_token,
                            scope_resolution_token,
                            subexpr,
                        },
                        module_item_path,
                    )
                }
                MajorEntityPath::ModuleItem(_) => todo!(),
            }
        } else {
            let MajorEntityPath::ModuleItem(module_item_path) = major_path else {
                todo!()
            };
            (
                ModuleItemPathExpr::Root {
                    name_token,
                    major_path,
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
