use super::*;

use husky_token::*;
use husky_token_data::db::{HasTokenDataDb, TokenDataDb};
use maybe_result::*;
use original_error::*;
use parsec::*;
use thiserror::Error;

/// major path expr is bottom-up
/// only module item path is allowed
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum MajorPathExprError {
    #[error("{0}")]
    Original(#[from] OriginalMajorPathExprError),
    #[error("{0}")]
    Derived(#[from] DerivedMajorPathExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum OriginalMajorPathExprError {
    #[error("unrecognized identifier")]
    UnrecognizedIdent(IdentToken),
    #[error("expect identifier")]
    ExpectedName(TokenStreamState),
    #[error("NoSuchSubitem")]
    NoSuchSubitem,
}

impl OriginalError for OriginalMajorPathExprError {
    type Error = MajorPathExprError;
}

impl From<TokenDataError> for MajorPathExprError {
    fn from(value: TokenDataError) -> Self {
        MajorPathExprError::Derived(value.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum DerivedMajorPathExprError {
    #[error("token error")]
    TokenData(#[from] TokenDataError),
}

pub type MajorItemPathExprResult<T> = Result<T, MajorPathExprError>;
pub type MajorItemPathExprMaybeResult<T> = MaybeResult<T, MajorPathExprError>;

pub(crate) struct MajorItemPathExprParser<'a, 'b> {
    db: &'a dyn EntitySynTreeDb,
    crate_root_path: ModulePath,
    token_stream: TokenStream<'a>,
    major_path_expr_arena: &'b mut MajorPathExprArena,
    item_tree_symbol_context: EntityTreeSymbolContext<'a, 'b>,
}

impl<'a, 'b> HasTokenDataDb<'a> for MajorItemPathExprParser<'a, 'b> {
    fn token_data_db(&self) -> &'a dyn TokenDataDb {
        self.db
    }
}

impl<'a, 'b> MajorItemPathExprParser<'a, 'b> {
    pub(crate) fn new(
        db: &'a dyn EntitySynTreeDb,
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
    ) -> MajorItemPathExprResult<(MajorItemPathExprIdx, MajorItemPath)> {
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
    ) -> MajorItemPathExprResult<(MajorItemPathExprIdx, MajorItemPath)> {
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
    ) -> MajorItemPathExprResult<(ArenaIdx<MajorItemPathExpr>, MajorItemPath)> {
        let (expr, module_item_path) =
            if let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonToken>() {
                match major_path {
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
                    MajorEntityPath::MajorItem(_) => todo!(),
                }
            } else {
                let MajorEntityPath::MajorItem(module_item_path) = major_path else {
                    todo!()
                };
                (
                    MajorItemPathExpr::Root {
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
