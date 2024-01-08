use super::*;
use husky_entity_syn_tree::HasSynNodePath;
use husky_hir_lazy_expr::{
    helpers::hir_lazy_expr_source_map_from_syn, HirLazyExprIdx, HirLazyStmtIdx,
};
use husky_regional_token::RegionalTokenIdxRange;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprDb};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};
use salsa::DebugWithDb;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValReprSource {
    ValItem(FugitivePath),
    Expansion {
        parent_val_repr: ValRepr,
        source: ValReprExpansionSource,
    },
}

impl ValReprSource {
    pub fn debug_info(self, db: &::salsa::Db) -> String {
        // ad hoc
        let extra = match self {
            ValReprSource::ValItem(_) => "".to_string(),
            ValReprSource::Expansion {
                parent_val_repr,
                source,
            } => format!(
                r#"regional_token_idx_range = {:?}"#,
                self.regional_token_idx_range(db)
            ),
        };
        format!(
            r#"self = {:?},
{extra}"#,
            self.debug(db)
        )
    }

    pub fn regional_token_idx_range(self, db: &::salsa::Db) -> RegionalTokenIdxRange {
        match self {
            ValReprSource::ValItem(_) => todo!(),
            ValReprSource::Expansion {
                parent_val_repr,
                source,
            } => match parent_val_repr.opn(db) {
                ValOpn::Require => todo!(),
                ValOpn::Assert => todo!(),
                ValOpn::ValItemLazilyDefined(path) => {
                    let ItemSynDefn {
                        syn_expr_region, ..
                    } = item_syn_defn(db, path.into()).unwrap();
                    let sema_expr_region = db.sema_expr_region(syn_expr_region);
                    let sema_expr_range_region_data =
                        sema_expr_range_region(db, sema_expr_region).data(db);
                    let source_map_data =
                        hir_lazy_expr_source_map_from_syn(syn_expr_region, db).data(db);
                    match source {
                        ValReprExpansionSource::LetVariable { stmt } => todo!(),
                        ValReprExpansionSource::RequireDefault { stmt } => todo!(),
                        ValReprExpansionSource::RequireCondition { stmt } => todo!(),
                        ValReprExpansionSource::AssertCondition { stmt } => todo!(),
                        ValReprExpansionSource::IfCondition { stmt } => todo!(),
                        ValReprExpansionSource::ElifCondition { stmt, branch_idx } => todo!(),
                        ValReprExpansionSource::Expr { expr } => {
                            sema_expr_range_region_data[source_map_data.sema_expr_idx(expr)]
                        }
                        ValReprExpansionSource::Stmt { stmt } => todo!(),
                    }
                }
                ValOpn::FunctionGn(_) => todo!(),
                ValOpn::Linkage(linkage) => todo!(),
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValReprExpansionSource {
    LetVariable {
        stmt: HirLazyStmtIdx,
    },
    RequireDefault {
        stmt: HirLazyStmtIdx,
    },
    RequireCondition {
        stmt: HirLazyStmtIdx,
    },
    AssertCondition {
        stmt: HirLazyStmtIdx,
    },
    IfCondition {
        stmt: HirLazyStmtIdx,
    },
    ElifCondition {
        stmt: HirLazyStmtIdx,
        branch_idx: u8,
    },
    Expr {
        expr: HirLazyExprIdx,
    },
    Stmt {
        stmt: HirLazyStmtIdx,
    },
}

impl ValReprSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            ValReprSource::ValItem(_) => ValCachingClass::ValItem,
            ValReprSource::Expansion { source, .. } => source.caching_class(),
        }
    }
}

impl ValReprExpansionSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            ValReprExpansionSource::LetVariable { .. } => ValCachingClass::Variable,
            ValReprExpansionSource::RequireDefault { .. } => ValCachingClass::Expr,
            ValReprExpansionSource::RequireCondition { .. }
            | ValReprExpansionSource::AssertCondition { .. }
            | ValReprExpansionSource::IfCondition { .. }
            | ValReprExpansionSource::ElifCondition { .. } => ValCachingClass::Condition,
            ValReprExpansionSource::Expr { .. } => ValCachingClass::Expr,
            ValReprExpansionSource::Stmt { .. } => ValCachingClass::Stmt,
        }
    }
}
