mod error;

pub use self::error::*;

use crate::*;
use husky_entity_path::MajorEntityPath;
use husky_entity_syn_tree::EntitySynTreeError;
use parsec::IsStreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum PrincipalEntityPathExpr {
    Root {
        path_name_token: PathNameRegionalToken,
        principal_entity_path: PrincipalEntityPath,
    },
    Subitem {
        parent: PrincipalEntityPathExprIdx,
        colon_colon_token: ColonColonRegionalToken,
        ident_token: PrincipalEntityPathExprResult<IdentRegionalToken>,
        path: PrincipalEntityPathExprResult<PrincipalEntityPath>,
    },
}

pub type SynPrincipalEntityPathExprArena = Arena<PrincipalEntityPathExpr>;
pub type PrincipalEntityPathExprIdx = ArenaIdx<PrincipalEntityPathExpr>;

// todo: change this to trait impl
impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_principal_item_path_expr(
        &mut self,
        path_name_token: PathNameRegionalToken,
        principal_item_path: PrincipalEntityPath,
    ) -> SynExpr {
        let root = self
            .context_mut()
            .alloc_item_path_expr(PrincipalEntityPathExpr::Root {
                path_name_token,
                principal_entity_path: principal_item_path,
            });
        if let Some(major_path) = principal_item_path.major()
           && let Some(colon_colon_token)= self.try_parse_err_as_none::<ColonColonRegionalToken>() {
            self.parse_subitem_path_expr(root, major_path, colon_colon_token)
        } else{
            SynExpr::PrincipalEntityPath {
                item_path_expr: root,
                opt_path: Some(principal_item_path),
            }
        }
    }

    fn parse_subitem_path_expr(
        &mut self,
        parent: PrincipalEntityPathExprIdx,
        parent_path: MajorEntityPath,
        colon_colon_regional_token: ColonColonRegionalToken,
    ) -> SynExpr {
        let ident_token: PrincipalEntityPathExprResult<IdentRegionalToken> = self
            .try_parse_expected(
                OriginalPrincipalEntityPathExprError::ExpectIdentAfterScopeResolution,
            );
        let path: PrincipalEntityPathExprResult<PrincipalEntityPath> = match ident_token {
            Ok(ident_token) => {
                let ident = ident_token.ident();
                match self.db().subitem_path(parent_path, ident) {
                    Ok(subitem_path) => match subitem_path {
                        SubitemPath::Principal(path) => Ok(path),
                        SubitemPath::Associated => {
                            return SynExpr::ScopeResolution {
                                parent_expr_idx: self.context_mut().alloc_expr(
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: parent,
                                        opt_path: Some(parent_path.into()),
                                    },
                                ),
                                colon_colon_regional_token,
                                ident_token,
                            }
                        }
                    },
                    Err(error) => Err(OriginalPrincipalEntityPathExprError::EntityTree {
                        regional_token_idx: ident_token.regional_token_idx(),
                        error,
                    }
                    .into()),
                }
            }
            Err(_) => todo!(),
        };
        let opt_path = path.as_ref().ok().copied();
        let expr = PrincipalEntityPathExpr::Subitem {
            parent,
            colon_colon_token: colon_colon_regional_token,
            ident_token,
            path,
        };
        let expr = self.context_mut().alloc_item_path_expr(expr);
        if let Some(path) = opt_path && let Some(major_path) = path.major()
            && let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonRegionalToken>() {
            self.parse_subitem_path_expr(expr, major_path, colon_colon_token)
        } else {
            SynExpr::PrincipalEntityPath {
                item_path_expr: expr,
                opt_path,
            }
        }
    }
}
