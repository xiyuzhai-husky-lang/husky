use crate::db::ValReprDb;
use crate::*;
#[cfg(test)]
use husky_hir_defn::HirDefn;
use husky_hir_lazy_expr::{
    variable::HirLazyVariableMap, HirLazyExpr, HirLazyExprIdx, HirLazyExprMap, HirLazyExprRegion,
    HirLazyExprRegionData, HirLazyStmt, HirLazyStmtIdx, HirLazyStmtIdxRange, HirLazyStmtMap,
};
use husky_val::ValOpr;
use husky_vfs::ModulePath;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, PartialEq, Eq)]
pub struct ValReprExpansion {
    hir_lazy_variable_val_repr_map: HirLazyVariableMap<ValRepr>,
    hir_lazy_expr_val_repr_map: HirLazyExprMap<ValRepr>,
    hir_lazy_stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
    root_hir_lazy_stmt_val_reprs: SmallVec<[ValRepr; 4]>,
}

#[salsa::tracked(jar = ValReprJar, return_ref)]
pub(crate) fn val_repr_expansion(
    db: &dyn ValReprDb,
    val_repr: ValRepr,
) -> Option<ValReprExpansion> {
    match val_repr.opr(db) {
        ValOpr::Fugitive(_) => todo!(),
        ValOpr::Require => None,
    }
}

pub struct ValReprExpansionBuilder<'a> {
    db: &'a dyn ValReprDb,
    domain_repr: ValDomainRepr,
    body: HirLazyExprIdx,
    hir_lazy_expr_region_data: HirLazyExprRegionData<'a>,
    // todo: change this to ordered map
    hir_lazy_variable_val_repr_map: HirLazyVariableMap<ValRepr>,
    hir_lazy_expr_val_repr_map: HirLazyExprMap<ValRepr>,
    hir_lazy_stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
    root_hir_lazy_stmt_val_reprs: SmallVec<[ValRepr; 4]>,
}

impl<'a> ValReprExpansionBuilder<'a> {
    fn new(
        db: &'a dyn ValReprDb,
        domain_repr: ValDomainRepr,
        body: HirLazyExprIdx,
        hir_lazy_expr_region: HirLazyExprRegion,
        argument_val_reprs: SmallVec<[ValRepr; 4]>,
    ) -> Self {
        let hir_lazy_expr_region_data = hir_lazy_expr_region.data(db);
        let mut variable_val_repr_map =
            HirLazyVariableMap::new(hir_lazy_expr_region_data.hir_lazy_variable_region().arena());
        for (hir_lazy_variable_idx, argument_val_repr) in std::iter::zip(
            hir_lazy_expr_region_data
                .hir_lazy_variable_region()
                .arena()
                .indices(),
            argument_val_reprs,
        ) {
            variable_val_repr_map.insert_new(hir_lazy_variable_idx, argument_val_repr)
        }
        Self {
            db,
            domain_repr,
            body,
            hir_lazy_expr_region_data,
            hir_lazy_variable_val_repr_map: variable_val_repr_map,
            hir_lazy_expr_val_repr_map: HirLazyExprMap::new2(
                hir_lazy_expr_region_data.hir_lazy_expr_arena(),
            ),
            hir_lazy_stmt_val_repr_map: HirLazyStmtMap::new2(
                hir_lazy_expr_region_data.hir_lazy_stmt_arena(),
            ),
            root_hir_lazy_stmt_val_reprs: smallvec![],
        }
    }

    fn build_all(&mut self) {
        let mut domain = ValDomainRepr::Omni;
        match self.hir_lazy_expr_region_data.hir_lazy_expr_arena()[self.body] {
            HirLazyExpr::Block { stmts } => {
                self.root_hir_lazy_stmt_val_reprs = self.build_stmts(stmts, self.domain_repr)
            }
            _ => todo!(),
        }
    }

    fn build_stmts(
        &mut self,
        stmts: HirLazyStmtIdxRange,
        mut domain_repr: ValDomainRepr,
    ) -> SmallVec<[ValRepr; 4]> {
        let mut val_reprs = smallvec![];
        for stmt in stmts {
            if let Some((val_repr, after_domain_repr)) = self.build_stmt(stmt, domain_repr) {
                domain_repr = after_domain_repr;
                val_reprs.push(val_repr)
            }
        }
        val_reprs
    }

    fn build_stmt(
        &mut self,
        stmt: HirLazyStmtIdx,
        domain_repr: ValDomainRepr,
    ) -> Option<(ValRepr, ValDomainRepr)> {
        match self.hir_lazy_expr_region_data.hir_lazy_stmt_arena()[stmt] {
            HirLazyStmt::Let {
                ref pattern,
                initial_value,
            } => todo!(),
            HirLazyStmt::Return { result } => {
                // ValRepr::new(db, opr, opds, domain, caching_strategy);
                todo!()
            }
            HirLazyStmt::Require { condition } => todo!(),
            HirLazyStmt::Assert { condition } => todo!(),
            HirLazyStmt::Eval { expr_idx } => todo!(),
            HirLazyStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            HirLazyStmt::Match {} => todo!(),
        }
    }

    fn finish(self) -> ValReprExpansion {
        ValReprExpansion {
            hir_lazy_variable_val_repr_map: self.hir_lazy_variable_val_repr_map,
            hir_lazy_expr_val_repr_map: self.hir_lazy_expr_val_repr_map,
            hir_lazy_stmt_val_repr_map: self.hir_lazy_stmt_val_repr_map,
            root_hir_lazy_stmt_val_reprs: self.root_hir_lazy_stmt_val_reprs,
        }
    }
}
