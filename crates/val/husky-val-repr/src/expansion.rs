use crate::db::ValReprDb;
use crate::*;
use husky_entity_path::{FugitivePath, MajorItemPath, PrincipalEntityPath};
#[cfg(test)]
use husky_hir_defn::HirDefn;
use husky_hir_defn::{FugitiveHirDefn, HasHirDefn};
use husky_hir_expr::{HirExprIdx, HirExprRegion};
use husky_hir_lazy_expr::{
    helpers::control_flow::HirLazyExprRegionControlFlowChart, variable::HirLazyVariableMap,
    HirLazyCallListItemGroup, HirLazyExpr, HirLazyExprIdx, HirLazyExprMap, HirLazyExprRegion,
    HirLazyExprRegionData, HirLazyStmt, HirLazyStmtIdx, HirLazyStmtIdxRange, HirLazyStmtMap,
};
use husky_hir_opr::suffix::HirSuffixOpr;
use husky_linkage_path::LinkagePath;
use husky_val::ValOpn;
use husky_vfs::ModulePath;
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValReprExpansion {
    #[return_ref]
    hir_lazy_variable_val_repr_map: HirLazyVariableMap<ValRepr>,
    #[return_ref]
    hir_lazy_expr_val_repr_map: HirLazyExprMap<ValRepr>,
    #[return_ref]
    hir_lazy_stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
    #[return_ref]
    root_hir_lazy_stmt_val_reprs: SmallVec<[ValRepr; 4]>,
}

impl ValRepr {
    pub fn expansion(self, db: &dyn ValReprDb) -> Option<ValReprExpansion> {
        val_repr_expansion(db, self)
    }
}

#[salsa::tracked(jar = ValReprJar)]
fn val_repr_expansion(db: &dyn ValReprDb, val_repr: ValRepr) -> Option<ValReprExpansion> {
    match val_repr.opr(db) {
        ValOpn::ValItem(fugitive_path) => {
            let FugitiveHirDefn::Val(hir_defn) = fugitive_path.hir_defn(db)? else {
                unreachable!()
            };
            debug_assert!(val_repr.opds(db).is_empty());
            let (HirExprIdx::Lazy(body), HirExprRegion::Lazy(hir_lazy_expr_region)) =
                hir_defn.body_with_hir_expr_region(db)?
            else {
                return None;
            };
            Some(build_val_repr_expansion(
                val_repr.val_domain_repr(db),
                body,
                hir_lazy_expr_region,
                &[],
                db,
            ))
        }
        ValOpn::FunctionGn(_) => todo!(),
        _ => None,
    }
}

fn build_val_repr_expansion(
    val_domain_repr: ValDomainRepr,
    body: HirLazyExprIdx,
    hir_lazy_expr_region: HirLazyExprRegion,
    argument_val_reprs: &[ValRepr],
    db: &dyn ValReprDb,
) -> ValReprExpansion {
    let mut builder = ValReprExpansionBuilder::new(
        val_domain_repr,
        body,
        hir_lazy_expr_region,
        argument_val_reprs,
        db,
    );
    builder.build_all();
    builder.finish()
}

struct ValReprExpansionBuilder<'a> {
    val_domain_repr: ValDomainRepr,
    body: HirLazyExprIdx,
    hir_lazy_expr_region_data: HirLazyExprRegionData<'a>,
    // todo: change this to ordered map
    hir_lazy_variable_val_repr_map: HirLazyVariableMap<ValRepr>,
    hir_lazy_expr_val_repr_map: HirLazyExprMap<ValRepr>,
    hir_lazy_stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
    root_hir_lazy_stmt_val_reprs: SmallVec<[ValRepr; 4]>,
    hir_lazy_expr_control_flow_region: &'a HirLazyExprRegionControlFlowChart,
    db: &'a dyn ValReprDb,
}

impl<'a> ValReprExpansionBuilder<'a> {
    fn new(
        val_domain_repr: ValDomainRepr,
        body: HirLazyExprIdx,
        hir_lazy_expr_region: HirLazyExprRegion,
        argument_val_reprs: &[ValRepr],
        db: &'a dyn ValReprDb,
    ) -> Self {
        let hir_lazy_expr_region_data = hir_lazy_expr_region.data(db);
        let mut variable_val_repr_map =
            HirLazyVariableMap::new(hir_lazy_expr_region_data.hir_lazy_variable_region().arena());
        for (hir_lazy_variable_idx, &argument_val_repr) in std::iter::zip(
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
            val_domain_repr,
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
            hir_lazy_expr_control_flow_region: hir_lazy_expr_region.control_flow(db),
        }
    }

    fn build_all(&mut self) {
        let mut domain = ValDomainRepr::Omni;
        match self.hir_lazy_expr_region_data.hir_lazy_expr_arena()[self.body] {
            HirLazyExpr::Block { stmts } => {
                self.root_hir_lazy_stmt_val_reprs = self.build_stmts(stmts, self.val_domain_repr)
            }
            _ => todo!(),
        }
    }

    fn build_stmts(
        &mut self,
        stmts: HirLazyStmtIdxRange,
        mut val_domain_repr: ValDomainRepr,
    ) -> SmallVec<[ValRepr; 4]> {
        let mut val_reprs = smallvec![];
        for stmt in stmts {
            if let Some((val_repr, after_domain_repr)) = self.build_stmt(val_domain_repr, stmt) {
                val_domain_repr = after_domain_repr;
                val_reprs.push(val_repr)
            }
        }
        val_reprs
    }

    fn build_stmt(
        &mut self,
        val_domain_repr: ValDomainRepr,
        stmt: HirLazyStmtIdx,
    ) -> Option<(ValRepr, ValDomainRepr)> {
        let (val_repr, val_domain_repr) =
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
                HirLazyStmt::Eval { expr_idx } => {
                    Some((self.build_expr(val_domain_repr, expr_idx), val_domain_repr))
                }
                HirLazyStmt::IfElse {
                    ref if_branch,
                    ref elif_branches,
                    ref else_branch,
                } => todo!(),
                HirLazyStmt::Match {} => todo!(),
            }?;
        self.hir_lazy_stmt_val_repr_map.insert_new(stmt, val_repr);
        Some((val_repr, val_domain_repr))
    }

    // exprs don't change domain
    fn build_expr(&mut self, val_domain_repr: ValDomainRepr, expr: HirLazyExprIdx) -> ValRepr {
        let hir_lazy_expr_arena = self.hir_lazy_expr_region_data.hir_lazy_expr_arena();
        let (opr, opds) = match hir_lazy_expr_arena[expr] {
            HirLazyExpr::Literal(_) => todo!(),
            HirLazyExpr::PrincipalEntityPath(_) => todo!(),
            HirLazyExpr::InheritedSynSymbol { ident } => todo!(),
            HirLazyExpr::CurrentSynSymbol { ident } => todo!(),
            HirLazyExpr::FrameVarDecl { ident } => todo!(),
            HirLazyExpr::Binary { lopd, opr, ropd } => {
                let opr = ValOpn::Binary(opr);
                let opds = smallvec![
                    self.build_expr(val_domain_repr, lopd),
                    self.build_expr(val_domain_repr, ropd)
                ];
                (opr, opds)
            }
            HirLazyExpr::Be { src, ref target } => todo!(),
            HirLazyExpr::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                let opr = ValOpn::Prefix(opr);
                let opds = smallvec![self.build_expr(val_domain_repr, opd_hir_expr_idx)];
                (opr, opds)
            }
            HirLazyExpr::Suffix {
                opd_hir_expr_idx,
                opr,
            } => {
                let opr = ValOpn::Suffix(opr);
                let opds = smallvec![self.build_expr(val_domain_repr, opd_hir_expr_idx)];
                (opr, opds)
            }
            HirLazyExpr::FnCall {
                function,
                ref generic_arguments,
                ref item_groups,
            } => match hir_lazy_expr_arena[function] {
                HirLazyExpr::PrincipalEntityPath(PrincipalEntityPath::MajorItem(
                    MajorItemPath::Fugitive(path),
                )) => {
                    let opr = ValOpn::Linkage(LinkagePath::new_item(self.db));
                    let mut opds: SmallVec<[ValRepr; 4]> = smallvec![];
                    for item_group in item_groups {
                        match *item_group {
                            HirLazyCallListItemGroup::Regular(hir_lazy_expr_idx) => {
                                opds.push(self.hir_lazy_expr_val_repr_map[hir_lazy_expr_idx])
                            }
                            HirLazyCallListItemGroup::Variadic => todo!(),
                            HirLazyCallListItemGroup::Keyed => todo!(),
                        }
                    }
                    (opr, opds)
                }
                HirLazyExpr::AssociatedFn => todo!(),
                _ => todo!(),
            },
            HirLazyExpr::GnCall {
                function,
                ref generic_arguments,
                ref item_groups,
            } => todo!(),
            HirLazyExpr::Field { owner, ident } => todo!(),
            HirLazyExpr::MethodFnCall {
                self_argument,
                ident,
                ref template_arguments,
                ref item_groups,
            } => todo!(),
            HirLazyExpr::NewTuple { ref items } => todo!(),
            HirLazyExpr::Index { owner, ref items } => todo!(),
            HirLazyExpr::List { ref items } => todo!(),
            HirLazyExpr::Block { stmts } => todo!(),
            HirLazyExpr::EmptyHtmlTag {
                function_ident,
                ref arguments,
            } => todo!(),
            HirLazyExpr::Todo => todo!(),
            HirLazyExpr::AssociatedFn => todo!(),
        };
        let val_repr = ValRepr::new(
            self.db,
            val_domain_repr,
            opr,
            opds,
            ValCachingStrategy::Skip,
        );
        self.hir_lazy_expr_val_repr_map.insert_new(expr, val_repr);
        val_repr
    }

    fn finish(self) -> ValReprExpansion {
        ValReprExpansion::new(
            self.db,
            self.hir_lazy_variable_val_repr_map,
            self.hir_lazy_expr_val_repr_map,
            self.hir_lazy_stmt_val_repr_map,
            self.root_hir_lazy_stmt_val_reprs,
        )
    }
}

#[cfg(test)]
fn val_item_val_repr_expansions(
    db: &DB,
    module_path: ModulePath,
) -> Vec<(FugitivePath, Option<ValReprExpansion>)> {
    val_item_val_reprs(db, module_path)
        .into_iter()
        .map(|(path, val_repr)| (path, val_repr.expansion(db)))
        .collect()
}

#[test]
fn val_item_val_repr_expansions_works() {
    // todo: why compiler needs this line to work?
    use husky_ast::test_utils::AstTestUtils;
    let db = DB::default();
    DB::default().ast_expect_test_debug_with_db(
        val_item_val_repr_expansions,
        &AstTestConfig::new("val_item_val_repr_expansions"),
    )
}
