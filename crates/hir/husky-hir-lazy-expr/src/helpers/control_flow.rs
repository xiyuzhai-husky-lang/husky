use std::ops::FromResidual;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub struct HirLazyExprRegionControlFlowChart {
    // has value means has control flow
    hir_lazy_expr_control_flow_chart: HirLazyExprMap<HasControlFlow>,
    // has value means has control flow
    hir_lazy_stmt_control_flow_chart: HirLazyStmtMap<HasControlFlow>,
}

impl HirLazyExprRegion {
    pub fn control_flow<'a>(
        self,
        db: &'a dyn HirLazyExprDb,
    ) -> &'a HirLazyExprRegionControlFlowChart {
        hir_lazy_expr_region_control_flow(db, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HasControlFlow {
    True = 1,
    False = -1,
}

impl HasControlFlow {
    fn to_bool(self) -> bool {
        self == HasControlFlow::True
    }
}

impl std::ops::Try for HasControlFlow {
    type Output = ();

    type Residual = HasControlFlowR;

    fn from_output(output: Self::Output) -> Self {
        Self::False
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HasControlFlow::True => std::ops::ControlFlow::Break(HasControlFlowR),
            HasControlFlow::False => std::ops::ControlFlow::Continue(()),
        }
    }
}

struct HasControlFlowR;

impl FromResidual<HasControlFlowR> for HasControlFlow {
    fn from_residual(residual: HasControlFlowR) -> Self {
        HasControlFlow::True
    }
}

#[salsa::tracked(jar = HirLazyExprJar, return_ref)]
fn hir_lazy_expr_region_control_flow(
    db: &dyn HirLazyExprDb,
    hir_lazy_expr_region: HirLazyExprRegion,
) -> HirLazyExprRegionControlFlowChart {
    let mut builder = HirLazyExprControlFlowRegionBuilder::new(hir_lazy_expr_region, db);
    builder.infer_all();
    builder.finish()
}

struct HirLazyExprControlFlowRegionBuilder<'a> {
    hir_lazy_expr_region_data: HirLazyExprRegionData<'a>,
    // has value means has control flow
    hir_lazy_expr_control_flow_chart: HirLazyExprMap<HasControlFlow>,
    // has value means has control flow
    hir_lazy_stmt_control_flow_chart: HirLazyStmtMap<HasControlFlow>,
}

impl<'a> HirLazyExprControlFlowRegionBuilder<'a> {
    fn new(hir_lazy_expr_region: HirLazyExprRegion, db: &'a dyn HirLazyExprDb) -> Self {
        let hir_lazy_expr_region_data = hir_lazy_expr_region.data(db);
        Self {
            hir_lazy_expr_region_data,
            hir_lazy_expr_control_flow_chart: HirLazyExprMap::new2(
                hir_lazy_expr_region_data.hir_lazy_expr_arena(),
            ),
            hir_lazy_stmt_control_flow_chart: HirLazyStmtMap::new2(
                hir_lazy_expr_region_data.hir_lazy_stmt_arena(),
            ),
        }
    }

    fn infer_all(&mut self) {
        for (hir_lazy_expr_idx, hir_lazy_expr) in self
            .hir_lazy_expr_region_data
            .hir_lazy_expr_arena()
            .indexed_iter()
        {
            self.infer_new_expr(hir_lazy_expr_idx, hir_lazy_expr);
        }
    }

    fn infer_new_expr(&mut self, hir_lazy_expr_idx: HirLazyExprIdx, hir_lazy_expr: &HirLazyExpr) {
        let has_control_flow = self.infer_new_expr_aux(hir_lazy_expr);
        self.hir_lazy_expr_control_flow_chart
            .insert_new(hir_lazy_expr_idx, has_control_flow)
    }

    fn infer_new_expr_aux(&mut self, hir_lazy_expr: &HirLazyExpr) -> HasControlFlow {
        match *hir_lazy_expr {
            HirLazyExpr::Literal(_)
            | HirLazyExpr::PrincipalEntityPath(_)
            | HirLazyExpr::InheritedSynSymbol { .. }
            | HirLazyExpr::CurrentSynSymbol { .. }
            | HirLazyExpr::FrameVarDecl { .. } => (),
            HirLazyExpr::Binary { lopd, ropd, .. } => {
                self.expr_has_control_flow(lopd)?;
                self.expr_has_control_flow(ropd)?
            }
            HirLazyExpr::Be { src, ref target } => self.expr_has_control_flow(src)?,
            HirLazyExpr::Prefix {
                opd_hir_expr_idx, ..
            } => self.expr_has_control_flow(opd_hir_expr_idx)?,
            HirLazyExpr::Suffix {
                opd_hir_expr_idx, ..
            } => self.expr_has_control_flow(opd_hir_expr_idx)?,
            HirLazyExpr::FnCall {
                function,
                ref generic_arguments,
                ref item_groups,
            } => {
                self.expr_has_control_flow(function)?;
                self.infer_new_item_groups(item_groups)?
            }
            HirLazyExpr::GnCall {
                function,
                ref generic_arguments,
                ref item_groups,
            } => {
                self.expr_has_control_flow(function)?;
                self.infer_new_item_groups(item_groups)?
            }
            HirLazyExpr::Field { owner, ident } => self.expr_has_control_flow(owner)?,
            HirLazyExpr::MethodFnCall {
                self_argument,
                ident,
                ref template_arguments,
                ref item_groups,
            } => {
                self.expr_has_control_flow(self_argument)?;
                self.infer_new_item_groups(item_groups)?
            }
            HirLazyExpr::NewTuple { ref items } => {
                for item in items {
                    todo!()
                }
            }
            HirLazyExpr::Index { owner, ref items } => {
                self.expr_has_control_flow(owner)?;
                self.infer_new_exprs(items)?
            }
            HirLazyExpr::List { ref items } => self.infer_new_exprs(items)?,
            HirLazyExpr::Block { stmts } => {
                for stmt in stmts {
                    self.infer_new_stmt(stmt)?
                }
            }
            HirLazyExpr::EmptyHtmlTag {
                function_ident,
                ref arguments,
            } => {
                for argument in arguments.iter() {
                    match *argument {
                        HirLazyHtmlArgumentExpr::Expanded { expr, .. } => {
                            self.expr_has_control_flow(expr)?
                        }
                        HirLazyHtmlArgumentExpr::Shortened { .. } => (),
                    }
                }
            }
            HirLazyExpr::Todo => (),
            HirLazyExpr::AssociatedFn => (),
        }
        HasControlFlow::False
    }

    fn infer_new_item_groups(
        &mut self,
        item_groups: &[HirLazyCallListItemGroup],
    ) -> HasControlFlow {
        for item_group in item_groups {
            match *item_group {
                HirLazyCallListItemGroup::Regular(item_expr_idx) => {
                    self.expr_has_control_flow(item_expr_idx)?
                }
                // ad hoc
                HirLazyCallListItemGroup::Variadic => (),
                // ad hoc
                HirLazyCallListItemGroup::Keyed => (),
            }
        }
        HasControlFlow::False
    }

    fn infer_new_exprs(&mut self, exprs: &[HirLazyExprIdx]) -> HasControlFlow {
        for &expr in exprs {
            self.expr_has_control_flow(expr)?
        }
        HasControlFlow::False
    }

    fn expr_has_control_flow(&self, hir_lazy_expr_idx: HirLazyExprIdx) -> HasControlFlow {
        self.hir_lazy_expr_control_flow_chart[hir_lazy_expr_idx]
    }

    fn infer_new_stmts(&mut self, stmts: HirLazyStmtIdxRange) -> HasControlFlow {
        for stmt in stmts {
            self.infer_new_stmt(stmt)?
        }
        HasControlFlow::False
    }

    fn infer_new_stmt(&mut self, hir_lazy_stmt: HirLazyStmtIdx) -> HasControlFlow {
        let has_control_flow = self.infer_new_stmt_aux(
            &self.hir_lazy_expr_region_data.hir_lazy_stmt_arena()[hir_lazy_stmt],
        );
        self.hir_lazy_stmt_control_flow_chart
            .insert_new(hir_lazy_stmt, has_control_flow);
        has_control_flow
    }

    fn infer_new_stmt_aux(&mut self, hir_lazy_stmt: &HirLazyStmt) -> HasControlFlow {
        match *hir_lazy_stmt {
            HirLazyStmt::Let {
                ref pattern,
                initial_value,
            } => self.expr_has_control_flow(initial_value),
            HirLazyStmt::Return { result } => HasControlFlow::True,
            HirLazyStmt::Require { condition } => HasControlFlow::True,
            HirLazyStmt::Assert { condition } => self.expr_has_control_flow(condition),
            HirLazyStmt::Eval { expr_idx } => self.expr_has_control_flow(expr_idx),
            HirLazyStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                self.infer_new_stmts(if_branch.stmts)?;
                for elif_branch in elif_branches {
                    self.infer_new_stmts(elif_branch.stmts)?;
                }
                if let Some(else_branch) = else_branch {
                    self.infer_new_stmts(else_branch.stmts)?
                }
                HasControlFlow::False
            }
            // ad hoc
            HirLazyStmt::Match {} => HasControlFlow::False,
        }
    }

    fn finish(mut self) -> HirLazyExprRegionControlFlowChart {
        HirLazyExprRegionControlFlowChart {
            hir_lazy_expr_control_flow_chart: self.hir_lazy_expr_control_flow_chart,
            hir_lazy_stmt_control_flow_chart: self.hir_lazy_stmt_control_flow_chart,
        }
    }
}
