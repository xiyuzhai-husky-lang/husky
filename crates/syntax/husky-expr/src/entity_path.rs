mod error;

pub use error::*;

use crate::*;
use husky_entity_tree::EntityTreeError;
use parsec::StreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum EntityPathExpr {
    Root {
        path_name_token: PathNameToken,
        entity_path: EntityPath,
    },
    Subentity {
        parent: EntityPathExprIdx,
        scope_resolution_token: ScopeResolutionToken,
        ident_token: EntityPathExprResult<IdentToken>,
        path: EntityPathExprResult<EntityPath>,
    },
}

pub type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;

// todo: change this to trait impl
impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_entity_path_expr(
        &mut self,
        path_name_token: PathNameToken,
        entity_path: EntityPath,
    ) -> Expr {
        let root = self.alloc_entity_path_expr(EntityPathExpr::Root {
            path_name_token,
            entity_path,
        });
        match self.try_parse_err_as_none::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(root, Some(entity_path), scope_resolution_token)
            }
            None => Expr::NonAssociatedEntityPath {
                entity_path_expr: root,
                path: Some(entity_path),
            },
        }
    }

    fn parse_subentity_path_expr(
        &mut self,
        parent: EntityPathExprIdx,
        parent_path: Option<EntityPath>,
        scope_resolution_token: ScopeResolutionToken,
    ) -> Expr {
        let ident_token: EntityPathExprResult<IdentToken> =
            self.try_parse_expected(OriginalEntityPathExprError::ExpectIdentAfterScopeResolution);
        let path: EntityPathExprResult<EntityPath> = match parent_path {
            Some(parent_path) => match ident_token {
                Ok(ident_token) => {
                    let ident = ident_token.ident();
                    match self.db().subentity_path(parent_path, ident) {
                        Ok(subentity_path) => match subentity_path {
                            SubentityPath::NonAssociated(path) => Ok(path),
                            SubentityPath::Associated => {
                                return Expr::AssociatedItemPath {
                                    parent: self.alloc_expr(Expr::NonAssociatedEntityPath {
                                        entity_path_expr: parent,
                                        path: Some(parent_path),
                                    }),
                                    scope_resolution_token,
                                    ident_token,
                                }
                            }
                        },
                        Err(error) => Err(OriginalEntityPathExprError::EntityTree {
                            token_idx: ident_token.token_idx(),
                            error,
                        }
                        .into()),
                    }
                }
                Err(_) => todo!(),
            },
            None => todo!(),
        };
        let parent_path = match path {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        let expr = EntityPathExpr::Subentity {
            parent,
            scope_resolution_token,
            ident_token,
            path,
        };
        let expr = self.alloc_entity_path_expr(expr);
        match self.try_parse_err_as_none::<ScopeResolutionToken>() {
            Some(scope_resolution_token) => {
                self.parse_subentity_path_expr(expr, parent_path, scope_resolution_token)
            }
            None => Expr::NonAssociatedEntityPath {
                entity_path_expr: expr,
                path: parent_path,
            },
        }
    }
}
