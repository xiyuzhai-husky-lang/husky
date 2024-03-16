use super::*;

use husky_token::*;
use maybe_result::*;
use original_error::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
/// only module item path is allowed
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum MajorItemPathExpr {
    Root {
        name_token: PathNameToken,
        major_path: MajorEntityPath,
    },
    Subitem {
        name_token: PathNameToken,
        colon_colon_token: ColonColonToken,
        subexpr: MajorItemPathExprIdx,
    },
}
pub type MajorPathExprArena = Arena<MajorItemPathExpr>;
pub type MajorItemPathExprIdx = ArenaIdx<MajorItemPathExpr>;
pub type MajorPathExprIdxRange = ArenaIdxRange<MajorItemPathExpr>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub enum MajorPathExprError {
    #[error("{0}")]
    Original(#[from] OriginalMajorItemPathExprError),
    #[error("{0}")]
    Derived(#[from] DerivedMajorPathExprError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub enum OriginalMajorItemPathExprError {
    #[error("unrecognized identifier")]
    UnrecognizedIdent(IdentToken),
    #[error("path is not major")]
    PathIsNotMajor {
        ident_token: IdentToken,
        path: PrincipalEntityPath,
    },
    #[error("expect identifier")]
    ExpectedName(TokenStreamState),
    #[error("NoSuchSubitem")]
    NoSuchSubitem,
    #[error("NoSuperForCrateRoot")]
    NoSuperForCrateRoot { super_token: SuperToken },
    #[error("NoSuperForParent")]
    NoSuperForParent {
        parent: ModulePath,
        super_token: SuperToken,
    },
    #[error("ExpectedMajorItemButGotModule")]
    ExpectedMajorItemButGotModule {
        name_token: PathNameToken,
        module_path: ModulePath,
    },
    #[error("NoColonColonAfterMajorItemPath")]
    NoColonColonAfterMajorItemPath,
}

impl OriginalError for OriginalMajorItemPathExprError {
    type Error = MajorPathExprError;
}

impl From<TokenDataError> for MajorPathExprError {
    fn from(value: TokenDataError) -> Self {
        MajorPathExprError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub enum DerivedMajorPathExprError {
    #[error("token error")]
    TokenData(#[from] TokenDataError),
}

pub type MajorItemPathExprResult<T> = Result<T, MajorPathExprError>;
pub type MajorItemPathExprMaybeResult<T> = MaybeResult<T, MajorPathExprError>;

pub(crate) struct MajorItemPathExprParser<'a, 'b> {
    db: &'a ::salsa::Db,
    crate_root_path: ModulePath,
    /// the module of the source code
    module_path: ModulePath,
    token_stream: TokenStream<'a>,
    /// context for resolving symbols
    item_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
    /// the storage of allocated major path expressions
    major_path_expr_arena: &'b mut MajorPathExprArena,
}

impl<'a, 'b> ::salsa::db::HasDb<'a> for MajorItemPathExprParser<'a, 'b> {
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }
}

impl<'a, 'b> MajorItemPathExprParser<'a, 'b> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        crate_root_path: ModulePath,
        module_path: ModulePath,
        token_stream: TokenStream<'a>,
        major_path_expr_arena: &'b mut MajorPathExprArena,
        item_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
    ) -> Self {
        Self {
            db,
            crate_root_path,
            module_path,
            token_stream,
            major_path_expr_arena,
            item_tree_symbol_context,
        }
    }
}

impl<'a, 'b> MajorItemPathExprParser<'a, 'b> {
    // todo: rollback
    pub(crate) fn parse_major_path_expr(
        &mut self,
    ) -> MajorItemPathExprMaybeResult<(MajorItemPathExprIdx, MajorItemPath)> {
        let name_token: PathNameToken = self.try_parse_option()??;
        let path = match name_token {
            PathNameToken::Ident(ident_token) => self
                .item_tree_symbol_context
                .resolve_root_ident(ident_token)?
                .principal_entity_path(self.db)
                .major()?,
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => self.module_path.into(),
            PathNameToken::Super(super_token) => match self.module_path.parent(self.db) {
                Some(parent) => parent.into(),
                None => Err(OriginalMajorItemPathExprError::NoSuperForCrateRoot { super_token })?,
            },
        };
        self.parse_major_path_expr_aux(path, name_token).into()
    }

    pub(crate) fn parse_major_path_expr_expected(
        &mut self,
    ) -> MajorItemPathExprResult<(MajorItemPathExprIdx, MajorItemPath)> {
        let name_token: PathNameToken =
            self.try_parse_expected(OriginalMajorItemPathExprError::ExpectedName)?;
        let path = match name_token {
            PathNameToken::Ident(ident_token) => {
                let path = self
                    .item_tree_symbol_context
                    .resolve_root_ident(ident_token)
                    .ok_or(OriginalMajorItemPathExprError::UnrecognizedIdent(
                        ident_token,
                    ))?
                    .principal_entity_path(self.db);
                path.major()
                    .ok_or(OriginalMajorItemPathExprError::PathIsNotMajor { ident_token, path })?
            }
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => self.module_path.into(),
            PathNameToken::Super(super_token) => match self.module_path.parent(self.db) {
                Some(parent) => parent.into(),
                None => Err(OriginalMajorItemPathExprError::NoSuperForCrateRoot { super_token })?,
            },
        };
        self.parse_major_path_expr_aux(path, name_token)
    }

    fn parse_major_path_subexpr(
        &mut self,
        parent: ModulePath,
    ) -> MajorItemPathExprResult<(MajorItemPathExprIdx, MajorItemPath)> {
        let db = self.db;
        let name_token: PathNameToken =
            self.try_parse_expected(OriginalMajorItemPathExprError::ExpectedName)?;
        let major_path = match name_token {
            PathNameToken::Ident(ident_token) => {
                let path = self
                    .item_tree_symbol_context
                    .resolve_subitem_symbol(parent.into(), ident_token.ident())
                    .ok_or(OriginalMajorItemPathExprError::NoSuchSubitem)?
                    .principal_entity_path(self.db);
                path.major()
                    .ok_or(OriginalMajorItemPathExprError::PathIsNotMajor { ident_token, path })?
            }
            PathNameToken::CrateRoot(_) => self.crate_root_path.into(),
            PathNameToken::SelfMod(_) => parent.into(),
            PathNameToken::Super(super_token) => parent
                .parent(db)
                .ok_or(OriginalMajorItemPathExprError::NoSuperForParent {
                    parent,
                    super_token,
                })?
                .into(),
        };
        self.parse_major_path_expr_aux(major_path, name_token)
    }

    fn parse_major_path_expr_aux(
        &mut self,
        major_entity_path: MajorEntityPath,
        name_token: PathNameToken,
    ) -> MajorItemPathExprResult<(MajorItemPathExprIdx, MajorItemPath)> {
        let (expr, module_item_path) =
            if let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonToken>() {
                match major_entity_path {
                    MajorEntityPath::Module(parent) => {
                        let (subexpr, module_item_path) = self.parse_major_path_subexpr(parent)?;
                        (
                            MajorItemPathExpr::Subitem {
                                name_token,
                                colon_colon_token,
                                subexpr,
                            },
                            module_item_path,
                        )
                    }
                    MajorEntityPath::MajorItem(_) => {
                        Err(OriginalMajorItemPathExprError::NoColonColonAfterMajorItemPath)?
                    }
                }
            } else {
                // we haven't seen a `::` after, so it's a root

                let major_item_path = match major_entity_path {
                    MajorEntityPath::Module(module_path) => Err(
                        OriginalMajorItemPathExprError::ExpectedMajorItemButGotModule {
                            name_token,
                            module_path,
                        },
                    )?,
                    MajorEntityPath::MajorItem(major_item_path) => major_item_path,
                };
                (
                    MajorItemPathExpr::Root {
                        name_token,
                        major_path: major_entity_path,
                    },
                    major_item_path,
                )
            };
        let expr = self.major_path_expr_arena.alloc_one(expr);
        Ok((expr, module_item_path))
    }
}

impl<'a, 'b> std::ops::Deref for MajorItemPathExprParser<'a, 'b> {
    type Target = TokenStream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for MajorItemPathExprParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for MajorItemPathExprParser<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for MajorItemPathExprParser<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b> StreamWrapper for MajorItemPathExprParser<'a, 'b> {}
