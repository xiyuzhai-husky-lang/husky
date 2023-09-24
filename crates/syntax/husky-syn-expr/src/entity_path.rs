mod identifiable_entity_path;
mod principal_entity_path;

pub use self::identifiable_entity_path::*;
pub use self::principal_entity_path::*;

use crate::*;
use husky_entity_path::MajorEntityPath;
use parsec::IsStreamParser;

// todo: change this to trait impl
impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_identifiable_item_path_expr(
        &mut self,
        path_name_token: PathNameRegionalToken,
        principal_item_path: PrincipalEntityPath,
    ) -> IdentifiableEntityPathExpr {
        let root = self
            .context_mut()
            .alloc_item_path_expr(PrincipalEntityPathExpr::Root {
                path_name_token,
                principal_entity_path: principal_item_path,
            });
        if let Some(major_path) = principal_item_path.major()
           && let Some(colon_colon_token)= self.try_parse_err_as_none::<ColonColonRegionalToken>() {
            self.parse_subitem_identifiable_path_expr(root, major_path, colon_colon_token)
        } else{
            IdentifiableEntityPathExpr::Principal  {
                item_path_expr: root,
                opt_path: Some(principal_item_path),
            }
        }
    }

    fn parse_subitem_identifiable_path_expr(
        &mut self,
        parent_expr_idx: PrincipalEntityPathExprIdx,
        parent_path: MajorEntityPath,
        colon_colon_regional_token: ColonColonRegionalToken,
    ) -> IdentifiableEntityPathExpr {
        let ident_token: SynExprResult<IdentRegionalToken> =
            self.try_parse_expected(OriginalSynExprError::ExpectIdentAfterScopeResolution);
        let path: SynExprResult<PrincipalEntityPath> = match ident_token {
            Ok(ident_token) => {
                let ident = ident_token.ident();
                match self.db().subitem_path(parent_path, ident) {
                    Ok(subitem_path) => match subitem_path {
                        SubitemPath::Principal(path) => Ok(path),
                        SubitemPath::Associated => {
                            return IdentifiableEntityPathExpr::ScopeResolution {
                                parent_expr_idx,
                                parent_path,
                                colon_colon_regional_token,
                                ident_token,
                            }
                        }
                    },
                    Err(error) => Err(OriginalSynExprError::EntityTree {
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
            parent: parent_expr_idx,
            colon_colon_token: colon_colon_regional_token,
            ident_token,
            path,
        };
        let expr = self.context_mut().alloc_item_path_expr(expr);
        if let Some(path) = opt_path && let Some(major_path) = path.major()
            && let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonRegionalToken>() {
            self.parse_subitem_identifiable_path_expr(expr, major_path, colon_colon_token)
        } else {
            IdentifiableEntityPathExpr::Principal  {
                item_path_expr: expr,
                opt_path,
            }
        }
    }
}
