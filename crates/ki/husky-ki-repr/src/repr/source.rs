use super::*;
use husky_entity_tree::HasSynNodePath;
use husky_hir_lazy_expr::{
    helpers::hir_lazy_expr_source_map_from_syn, HirLazyExprIdx, HirLazyStmtIdx,
};
use husky_regional_token::RegionalTokenIdxRange;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprDb};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};
use salsa::DebugWithDb;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KiReprSource {
    ValItem(FugitivePath),
    Expansion {
        parent_ki_repr: KiRepr,
        source: KiReprExpansionSource,
    },
}

impl KiReprSource {
    pub fn debug_info(self, db: &::salsa::Db) -> String {
        // ad hoc
        let extra = match self {
            KiReprSource::ValItem(_) => "".to_string(),
            KiReprSource::Expansion {
                parent_ki_repr,
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
            KiReprSource::ValItem(_) => todo!(),
            KiReprSource::Expansion {
                parent_ki_repr,
                source,
            } => match parent_ki_repr.opn(db) {
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
                        KiReprExpansionSource::LetVariable { stmt } => todo!(),
                        KiReprExpansionSource::RequireDefault { stmt } => todo!(),
                        KiReprExpansionSource::RequireCondition { stmt } => todo!(),
                        KiReprExpansionSource::AssertCondition { stmt } => todo!(),
                        KiReprExpansionSource::IfCondition { stmt } => todo!(),
                        KiReprExpansionSource::ElifCondition { stmt, branch_idx } => todo!(),
                        KiReprExpansionSource::Expr { expr } => {
                            sema_expr_range_region_data[source_map_data.sema_expr_idx(expr)]
                        }
                        KiReprExpansionSource::Stmt { stmt } => todo!(),
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
pub enum KiReprExpansionSource {
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

impl KiReprSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            KiReprSource::ValItem(_) => ValCachingClass::ValItem,
            KiReprSource::Expansion { source, .. } => source.caching_class(),
        }
    }
}

impl KiReprExpansionSource {
    pub(crate) fn caching_class(self) -> ValCachingClass {
        match self {
            KiReprExpansionSource::LetVariable { .. } => ValCachingClass::Variable,
            KiReprExpansionSource::RequireDefault { .. } => ValCachingClass::Expr,
            KiReprExpansionSource::RequireCondition { .. }
            | KiReprExpansionSource::AssertCondition { .. }
            | KiReprExpansionSource::IfCondition { .. }
            | KiReprExpansionSource::ElifCondition { .. } => ValCachingClass::Condition,
            KiReprExpansionSource::Expr { .. } => ValCachingClass::Expr,
            KiReprExpansionSource::Stmt { .. } => ValCachingClass::Stmt,
        }
    }
}
