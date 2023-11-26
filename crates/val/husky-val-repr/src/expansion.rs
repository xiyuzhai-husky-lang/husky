use crate::{db::ValReprDb, *};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{FugitivePath, MajorItemPath, PrincipalEntityPath};

use husky_hir_defn::{FugitiveHirDefn, HasHirDefn};
use husky_hir_expr::{HirExprIdx, HirExprRegion};
use husky_hir_lazy_expr::{
    helpers::control_flow::{HasControlFlow, HirLazyExprRegionControlFlowChart},
    variable::HirLazyVariableMap,
    HirLazyCallListItemGroup, HirLazyExprData, HirLazyExprIdx, HirLazyExprMap, HirLazyExprRegion,
    HirLazyExprRegionData, HirLazyPatternExpr, HirLazyStmt, HirLazyStmtIdx, HirLazyStmtIdxRange,
    HirLazyStmtMap,
};

use husky_linkage::linkage::Linkage;

use husky_val::ValOpn;
use husky_vfs::ModulePath;
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValReprExpansion {
    #[return_ref]
    pub hir_lazy_variable_val_repr_map: HirLazyVariableMap<ValRepr>,
    #[return_ref]
    pub hir_lazy_expr_val_repr_map: HirLazyExprMap<ValRepr>,
    #[return_ref]
    pub hir_lazy_stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
    #[return_ref]
    pub root_hir_lazy_stmt_val_reprs: SmallVec<[ValRepr; 4]>,
}

impl ValRepr {
    pub fn expansion(self, db: &dyn ValReprDb) -> Option<ValReprExpansion> {
        val_repr_expansion(db, self)
    }
}

#[salsa::tracked(jar = ValReprJar)]
fn val_repr_expansion(db: &dyn ValReprDb, val_repr: ValRepr) -> Option<ValReprExpansion> {
    match val_repr.opn(db) {
        ValOpn::ValItem(fugitive_path) => {
            let FugitiveHirDefn::Val(hir_defn) = fugitive_path.hir_defn(db)? else {
                unreachable!()
            };
            debug_assert!(val_repr.arguments(db).is_empty());
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
        let val_domain_repr_guard = ValDomainReprGuard::new(self.db, self.val_domain_repr);
        match self.hir_lazy_expr_region_data.hir_lazy_expr_arena()[self.body] {
            HirLazyExprData::Block { stmts } => {
                self.root_hir_lazy_stmt_val_reprs = self.build_stmts(val_domain_repr_guard, stmts)
            }
            _ => todo!(),
        }
    }

    fn build_stmts(
        &mut self,
        mut val_domain_repr_guard: ValDomainReprGuard<'a>,
        stmts: HirLazyStmtIdxRange,
    ) -> SmallVec<[ValRepr; 4]> {
        let mut val_reprs = smallvec![];
        for stmt in stmts {
            if let Some(val_repr) = self.build_stmt(&mut val_domain_repr_guard, stmt) {
                val_domain_repr_guard.after_stmt(val_repr);
                val_reprs.push(val_repr)
            }
        }
        val_reprs
    }

    fn build_stmt(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        stmt: HirLazyStmtIdx,
    ) -> Option<ValRepr> {
        let (opn, arguments) = match self.hir_lazy_expr_region_data.hir_lazy_stmt_arena()[stmt] {
            HirLazyStmt::Let {
                ref pattern,
                initial_value,
            } => {
                let initial_value_val_repr = self.build_expr(val_domain_repr_guard, initial_value);
                match self.hir_lazy_expr_region_data.hir_lazy_pattern_expr_arena()
                    [pattern.pattern_expr_idx()]
                {
                    HirLazyPatternExpr::Literal(_) => todo!(),
                    HirLazyPatternExpr::Ident { ident: _ } => {
                        debug_assert_eq!(pattern.variables().len(), 1);
                        self.hir_lazy_variable_val_repr_map.insert_new(
                            pattern.variables()[0],
                            initial_value_val_repr
                                .with_caching_class(ValCachingClass::Variable, self.db),
                        );
                        return None;
                    }
                    HirLazyPatternExpr::Unit(_) => todo!(),
                    HirLazyPatternExpr::Tuple { path: _, fields: _ } => todo!(),
                    HirLazyPatternExpr::Props { path: _, fields: _ } => todo!(),
                    HirLazyPatternExpr::OneOf { options: _ } => todo!(),
                    HirLazyPatternExpr::Binding { ident: _, src: _ } => todo!(),
                    HirLazyPatternExpr::Range { start: _, end: _ } => todo!(),
                }
            }
            HirLazyStmt::Return { result } => (
                ValOpn::Return,
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, result)
                )],
            ),
            HirLazyStmt::Require { condition } => (
                ValOpn::Require,
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, condition)
                )],
            ),
            HirLazyStmt::Assert { condition } => (
                ValOpn::Assert,
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, condition)
                )],
            ),
            HirLazyStmt::Eval {
                expr_idx,
                discarded,
            } => {
                let expr_val_repr = self.build_expr(val_domain_repr_guard, expr_idx);
                match discarded {
                    true => match self.hir_lazy_expr_control_flow_region[stmt] {
                        HasControlFlow::True => (
                            ValOpn::EvalDiscarded,
                            smallvec![ValArgumentRepr::Ordinary(expr_val_repr)],
                        ),
                        HasControlFlow::False => return None,
                    },
                    false => return Some(expr_val_repr),
                }
            }
            HirLazyStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let mut val_domain_repr_guard = val_domain_repr_guard.clone();
                let mut branches: SmallVec<[ValArgumentRepr; 4]> = smallvec![];
                let if_condition =
                    self.build_expr(&mut val_domain_repr_guard, if_branch.condition());
                branches.push(ValArgumentRepr::Branch {
                    condition: Some(if_condition),
                    stmts: self.build_stmts(
                        val_domain_repr_guard.under_condition(if_condition),
                        if_branch.stmts(),
                    ),
                });
                for elif_branch in elif_branches {
                    let elif_condition =
                        self.build_expr(&mut val_domain_repr_guard, elif_branch.condition());
                    branches.push(ValArgumentRepr::Branch {
                        condition: Some(elif_condition),
                        stmts: self.build_stmts(
                            val_domain_repr_guard.under_condition(elif_condition),
                            elif_branch.stmts(),
                        ),
                    });
                }
                if let Some(else_branch) = else_branch {
                    branches.push(ValArgumentRepr::Branch {
                        condition: None,
                        stmts: self.build_stmts(val_domain_repr_guard, else_branch.stmts()),
                    });
                }
                (ValOpn::Branches, branches)
            }
            HirLazyStmt::Match {} => todo!(),
        };
        let val_repr = val_domain_repr_guard.new_stmt_val_repr(opn, arguments);
        self.hir_lazy_stmt_val_repr_map.insert_new(stmt, val_repr);
        Some(val_repr)
    }

    fn build_expr(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        expr: HirLazyExprIdx,
    ) -> ValRepr {
        let val_repr = self.build_expr_aux(val_domain_repr_guard, expr);
        self.hir_lazy_expr_val_repr_map.insert_new(expr, val_repr);
        val_repr
    }

    fn build_expr_aux(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        expr: HirLazyExprIdx,
    ) -> ValRepr {
        let hir_lazy_expr_arena = self.hir_lazy_expr_region_data.hir_lazy_expr_arena();
        let (opn, arguments) = match hir_lazy_expr_arena[expr] {
            HirLazyExprData::Literal(lit) => (ValOpn::Literal(lit), smallvec![]),
            HirLazyExprData::PrincipalEntityPath(path) => match path {
                PrincipalEntityPath::Module(_) => todo!(),
                PrincipalEntityPath::MajorItem(path) => match path {
                    MajorItemPath::Type(_) => todo!(),
                    MajorItemPath::Trait(_) => todo!(),
                    MajorItemPath::Fugitive(path) => match path.fugitive_kind(self.db) {
                        FugitiveKind::FunctionFn => todo!(),
                        FugitiveKind::FunctionGn => todo!(),
                        FugitiveKind::AliasType => todo!(),
                        FugitiveKind::Val => return ValRepr::new_val_item(path, self.db),
                    },
                },
                PrincipalEntityPath::TypeVariant(path) => (ValOpn::TypeVariant(path), smallvec![]),
            },
            HirLazyExprData::ConstSymbol(_) => todo!(),
            HirLazyExprData::Variable(var) => return self.hir_lazy_variable_val_repr_map[var],
            HirLazyExprData::Binary { lopd, opr, ropd } => {
                let opn = ValOpn::Binary(opr);
                let arguments = smallvec![
                    ValArgumentRepr::Ordinary(self.build_expr(val_domain_repr_guard, lopd)),
                    ValArgumentRepr::Ordinary(self.build_expr(val_domain_repr_guard, ropd))
                ];
                (opn, arguments)
            }
            HirLazyExprData::Be { src, target: _ } => (
                ValOpn::Be,
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, src)
                )],
            ),
            HirLazyExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                let opn = ValOpn::Prefix(opr);
                let arguments = smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, opd_hir_expr_idx)
                )];
                (opn, arguments)
            }
            HirLazyExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => {
                let opn = ValOpn::Suffix(opr);
                let arguments = smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, opd_hir_expr_idx)
                )];
                (opn, arguments)
            }
            HirLazyExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::LinkageImpl(Linkage::new_item(path, instantiation, self.db));
                let mut arguments: SmallVec<[ValArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(item_groups, val_domain_repr_guard, &mut arguments);
                (opn, arguments)
            }
            HirLazyExprData::TypeVariantConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::LinkageImpl(Linkage::new_item(path, instantiation, self.db));
                let mut arguments: SmallVec<[ValArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(item_groups, val_domain_repr_guard, &mut arguments);
                (opn, arguments)
            }
            HirLazyExprData::FunctionFnItemCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::LinkageImpl(Linkage::new_item(path, instantiation, self.db));
                let mut arguments: SmallVec<[ValArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(item_groups, val_domain_repr_guard, &mut arguments);
                (opn, arguments)
            }
            HirLazyExprData::AssociatedFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::LinkageImpl(Linkage::new_item(path, instantiation, self.db));
                let mut arguments: SmallVec<[ValArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(item_groups, val_domain_repr_guard, &mut arguments);
                (opn, arguments)
            }
            HirLazyExprData::FunctionGnItemCall { .. } => todo!(),
            HirLazyExprData::PropsStructField { owner, .. } => (
                ValOpn::LinkageImpl(Linkage::new_props_struct_field(self.db)),
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, owner)
                )],
            ),
            HirLazyExprData::MemoizedField {
                owner,
                ref indirections,
                ref instantiation,
                ..
            } => (
                ValOpn::LinkageImpl(Linkage::new_memoized_field(self.db)),
                smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, owner)
                )],
            ),
            HirLazyExprData::MethodFnCall {
                self_argument,
                ref indirections,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let mut arguments = smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, self_argument)
                )];
                self.build_item_groups(item_groups, val_domain_repr_guard, &mut arguments);
                (ValOpn::LinkageImpl(Linkage::new_method(self.db)), arguments)
            }
            HirLazyExprData::NewTuple { items: _ } => todo!(),
            HirLazyExprData::Index { owner, ref items } => {
                let mut arguments = smallvec![ValArgumentRepr::Ordinary(
                    self.build_expr(val_domain_repr_guard, owner)
                )];
                for &item in items {
                    arguments.push(ValArgumentRepr::Ordinary(
                        self.build_expr(val_domain_repr_guard, item),
                    ))
                }
                (ValOpn::LinkageImpl(Linkage::new_index(self.db)), arguments)
            }
            HirLazyExprData::NewList { ref items } => (
                ValOpn::NewList,
                items
                    .iter()
                    .map(|&item| {
                        ValArgumentRepr::Ordinary(self.build_expr(val_domain_repr_guard, item))
                    })
                    .collect(),
            ),
            HirLazyExprData::Block { stmts: _ } => todo!(),
            HirLazyExprData::EmptyHtmlTag {
                function_ident: _,
                arguments: _,
            } => todo!(),
            HirLazyExprData::Todo => todo!(),
            HirLazyExprData::Unreachable => todo!(),
            HirLazyExprData::AssociatedFn { .. } => todo!(),
        };
        val_domain_repr_guard.new_expr_val_repr(
            opn,
            arguments,
            self.hir_lazy_expr_control_flow_region[expr],
        )
    }

    fn build_item_groups(
        &mut self,
        item_groups: &[HirLazyCallListItemGroup],
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        arguments: &mut SmallVec<[ValArgumentRepr; 4]>,
    ) {
        for item_group in item_groups {
            match *item_group {
                HirLazyCallListItemGroup::Regular(item) => arguments.push(
                    ValArgumentRepr::Ordinary(self.build_expr(val_domain_repr_guard, item)),
                ),
                HirLazyCallListItemGroup::Variadic(ref items) => {
                    let items: Vec<_> = items
                        .iter()
                        .map(|&item| self.build_expr(val_domain_repr_guard, item))
                        .collect();
                    arguments.push(ValArgumentRepr::Variadic(items))
                }
                HirLazyCallListItemGroup::Keyed(key, item) => {
                    arguments.push(ValArgumentRepr::Keyed(
                        key,
                        item.map(|item| self.build_expr(val_domain_repr_guard, item)),
                    ))
                }
            }
        }
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
    db: &::salsa::Db,
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
    DB::default().ast_expect_test_debug_with_db(
        val_item_val_repr_expansions,
        &AstTestConfig::new("val_item_val_repr_expansions"),
    )
}
