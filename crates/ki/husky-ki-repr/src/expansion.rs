use crate::{
    repr::source::{KiReprExpansionSource, KiReprSource},
    *,
};
use husky_entity_kind::MajorFugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};

use husky_hir_defn::{FugitiveHirDefn, HasHirDefn};
use husky_hir_expr::{HirExprIdx, HirExprRegion};
use husky_hir_lazy_expr::{
    helpers::control_flow::{HasControlFlow, HirLazyExprRegionControlFlowChart},
    variable::HirLazyVariableMap,
    HirLazyBeVariablesPattern, HirLazyCallListArgument, HirLazyCondition, HirLazyExprData,
    HirLazyExprIdx, HirLazyExprMap, HirLazyExprRegion, HirLazyExprRegionData, HirLazyPatternExpr,
    HirLazyStmtData, HirLazyStmtIdx, HirLazyStmtIdxRange, HirLazyStmtMap,
};

use husky_hir_ty::{
    instantiation::{HirInstantiation, HirTermSvarResolution},
    HirConstant, HirTemplateArgument, HirTemplateSvar, HirTemplateSvarClass,
};
use husky_linkage::{instantiation::LinInstantiation, linkage::Linkage};

use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData, ValOpn, ValPatternData};
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = KiReprDb, jar = KiReprJar)]
pub struct KiReprExpansion {
    #[return_ref]
    pub hir_lazy_variable_ki_repr_map: HirLazyVariableMap<KiRepr>,
    #[return_ref]
    pub hir_lazy_expr_ki_repr_map: HirLazyExprMap<KiRepr>,
    #[return_ref]
    pub hir_lazy_stmt_ki_repr_map: HirLazyStmtMap<KiRepr>,
    #[return_ref]
    pub root_hir_lazy_stmt_ki_reprs: SmallVec<[KiRepr; 4]>,
}

impl KiRepr {
    pub fn expansion(self, db: &::salsa::Db) -> Option<KiReprExpansion> {
        ki_repr_expansion(db, self)
    }
}

#[salsa::tracked(jar = KiReprJar)]
fn ki_repr_expansion(db: &::salsa::Db, ki_repr: KiRepr) -> Option<KiReprExpansion> {
    match ki_repr.opn(db) {
        ValOpn::ValItemLazilyDefined(fugitive_path) => {
            let FugitiveHirDefn::Ki(hir_defn) = fugitive_path.hir_defn(db)? else {
                unreachable!()
            };
            debug_assert!(ki_repr.arguments(db).is_empty());
            let (HirExprIdx::Lazy(body), HirExprRegion::Lazy(hir_lazy_expr_region)) =
                hir_defn.hir_expr_body_and_region(db)?
            else {
                return None;
            };
            Some(build_ki_repr_expansion(
                ki_repr,
                body,
                hir_lazy_expr_region,
                &[],
                LinInstantiation::new_empty(false),
                db,
            ))
        }
        ValOpn::FunctionGn(_) => todo!(),
        _ => None,
    }
}

fn build_ki_repr_expansion(
    parent_ki_repr: KiRepr,
    body: HirLazyExprIdx,
    hir_lazy_expr_region: HirLazyExprRegion,
    argument_ki_reprs: &[KiRepr],
    lin_instantiation: LinInstantiation,
    db: &::salsa::Db,
) -> KiReprExpansion {
    let mut builder = KiReprExpansionBuilder::new(
        parent_ki_repr,
        body,
        hir_lazy_expr_region,
        argument_ki_reprs,
        lin_instantiation,
        db,
    );
    builder.build_all();
    builder.finish()
}

// todo: lin_instantiation
struct KiReprExpansionBuilder<'a> {
    parent_ki_repr: KiRepr,
    val_domain_repr: ValDomainRepr,
    body: HirLazyExprIdx,
    hir_lazy_expr_region_data: HirLazyExprRegionData<'a>,
    // todo: change this to ordered map
    hir_lazy_variable_ki_repr_map: HirLazyVariableMap<KiRepr>,
    hir_lazy_expr_ki_repr_map: HirLazyExprMap<KiRepr>,
    hir_lazy_stmt_ki_repr_map: HirLazyStmtMap<KiRepr>,
    root_hir_lazy_stmt_ki_reprs: SmallVec<[KiRepr; 4]>,
    hir_lazy_expr_control_flow_region: &'a HirLazyExprRegionControlFlowChart,
    lin_instantiation: LinInstantiation,
    db: &'a ::salsa::Db,
}

impl<'a> KiReprExpansionBuilder<'a> {
    fn new(
        parent_ki_repr: KiRepr,
        body: HirLazyExprIdx,
        hir_lazy_expr_region: HirLazyExprRegion,
        argument_ki_reprs: &[KiRepr],
        lin_instantiation: LinInstantiation,
        db: &'a ::salsa::Db,
    ) -> Self {
        let hir_lazy_expr_region_data = hir_lazy_expr_region.data(db);
        let mut variable_ki_repr_map =
            HirLazyVariableMap::new(hir_lazy_expr_region_data.hir_lazy_variable_region().arena());
        for (hir_lazy_variable_idx, &argument_ki_repr) in std::iter::zip(
            hir_lazy_expr_region_data
                .hir_lazy_variable_region()
                .arena()
                .indices(),
            argument_ki_reprs,
        ) {
            variable_ki_repr_map.insert_new(hir_lazy_variable_idx, argument_ki_repr)
        }
        Self {
            parent_ki_repr,
            val_domain_repr: parent_ki_repr.val_domain_repr(db),
            body,
            hir_lazy_expr_region_data,
            hir_lazy_variable_ki_repr_map: variable_ki_repr_map,
            hir_lazy_expr_ki_repr_map: HirLazyExprMap::new2(
                hir_lazy_expr_region_data.hir_lazy_expr_arena(),
            ),
            hir_lazy_stmt_ki_repr_map: HirLazyStmtMap::new2(
                hir_lazy_expr_region_data.hir_lazy_stmt_arena(),
            ),
            root_hir_lazy_stmt_ki_reprs: smallvec![],
            hir_lazy_expr_control_flow_region: hir_lazy_expr_region.control_flow(db),
            lin_instantiation,
            db,
        }
    }

    fn build_all(&mut self) {
        let val_domain_repr_guard =
            ValDomainReprGuard::new(self.db, self.parent_ki_repr, self.val_domain_repr);
        match self.hir_lazy_expr_region_data.hir_lazy_expr_arena()[self.body] {
            HirLazyExprData::Block { stmts } => {
                self.root_hir_lazy_stmt_ki_reprs = self.build_stmts(val_domain_repr_guard, stmts)
            }
            _ => todo!(),
        }
    }

    fn build_stmts(
        &mut self,
        mut val_domain_repr_guard: ValDomainReprGuard<'a>,
        stmts: HirLazyStmtIdxRange,
    ) -> SmallVec<[KiRepr; 4]> {
        let mut ki_reprs = smallvec![];
        for stmt in stmts {
            if let Some(ki_repr) = self.build_stmt(&mut val_domain_repr_guard, stmt) {
                val_domain_repr_guard.after_stmt(ki_repr);
                ki_reprs.push(ki_repr)
            }
        }
        ki_reprs
    }

    fn build_stmt(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        stmt: HirLazyStmtIdx,
    ) -> Option<KiRepr> {
        let (opn, arguments) = match self.hir_lazy_expr_region_data.hir_lazy_stmt_arena()[stmt] {
            HirLazyStmtData::Let {
                ref pattern,
                initial_value,
            } => {
                let initial_value_ki_repr = self.build_expr(val_domain_repr_guard, initial_value);
                match self.hir_lazy_expr_region_data.hir_lazy_pattern_expr_arena()
                    [pattern.pattern_expr_idx()]
                {
                    HirLazyPatternExpr::Literal(_) => todo!(),
                    HirLazyPatternExpr::Ident { ident: _ } => {
                        debug_assert_eq!(pattern.variables().len(), 1);
                        self.hir_lazy_variable_ki_repr_map.insert_new(
                            pattern.variables()[0],
                            initial_value_ki_repr.with_source(
                                KiReprSource::Expansion {
                                    parent_ki_repr: self.parent_ki_repr,
                                    source: KiReprExpansionSource::LetVariable { stmt },
                                },
                                self.db,
                            ),
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
            HirLazyStmtData::Return { result } => (
                ValOpn::Return,
                smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, result)
                )],
            ),
            HirLazyStmtData::Require {
                ref condition,
                return_ty,
            } => {
                let db = self.db;
                let default = val_domain_repr_guard.new_ki_repr(
                    KiReprExpansionSource::RequireDefault { stmt },
                    ValOpn::Linkage(Linkage::new_ty_default(
                        return_ty,
                        &self.lin_instantiation,
                        db,
                    )),
                    smallvec![],
                    HasControlFlow::False,
                );
                (
                    ValOpn::Require,
                    smallvec![
                        KiArgumentRepr::Simple(self.build_condition(
                            val_domain_repr_guard,
                            KiReprExpansionSource::RequireCondition { stmt },
                            condition
                        )),
                        KiArgumentRepr::Simple(default)
                    ],
                )
            }
            HirLazyStmtData::Assert { ref condition } => (
                ValOpn::Assert,
                smallvec![KiArgumentRepr::Simple(self.build_condition(
                    val_domain_repr_guard,
                    KiReprExpansionSource::AssertCondition { stmt },
                    condition
                ))],
            ),
            HirLazyStmtData::Eval {
                expr_idx,
                discarded,
            } => {
                let expr_ki_repr = self.build_expr(val_domain_repr_guard, expr_idx);
                match discarded {
                    true => match self.hir_lazy_expr_control_flow_region[stmt] {
                        HasControlFlow::True => (
                            ValOpn::EvalDiscarded,
                            smallvec![KiArgumentRepr::Simple(expr_ki_repr)],
                        ),
                        HasControlFlow::False => return None,
                    },
                    false => return Some(expr_ki_repr),
                }
            }
            HirLazyStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let mut val_domain_repr_guard = val_domain_repr_guard.clone();
                let mut branches: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                let if_condition = self.build_condition(
                    &mut val_domain_repr_guard,
                    KiReprExpansionSource::IfCondition { stmt },
                    &if_branch.condition,
                );
                branches.push(KiArgumentRepr::Branch {
                    condition: Some(if_condition),
                    stmts: self.build_stmts(
                        val_domain_repr_guard.under_condition(if_condition),
                        if_branch.stmts,
                    ),
                });
                for (branch_idx, elif_branch) in elif_branches.iter().enumerate() {
                    let elif_condition = self.build_condition(
                        &mut val_domain_repr_guard,
                        KiReprExpansionSource::ElifCondition {
                            stmt,
                            branch_idx: branch_idx.try_into().unwrap(),
                        },
                        &elif_branch.condition,
                    );
                    branches.push(KiArgumentRepr::Branch {
                        condition: Some(elif_condition),
                        stmts: self.build_stmts(
                            val_domain_repr_guard.under_condition(elif_condition),
                            elif_branch.stmts,
                        ),
                    });
                }
                if let Some(else_branch) = else_branch {
                    branches.push(KiArgumentRepr::Branch {
                        condition: None,
                        stmts: self.build_stmts(val_domain_repr_guard, else_branch.stmts()),
                    });
                }
                (ValOpn::Branches, branches)
            }
            HirLazyStmtData::Match {} => todo!(),
        };
        let ki_repr = val_domain_repr_guard.new_stmt_ki_repr(stmt, opn, arguments);
        self.hir_lazy_stmt_ki_repr_map.insert_new(stmt, ki_repr);
        Some(ki_repr)
    }

    fn build_condition(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        source: KiReprExpansionSource,
        condition: &HirLazyCondition,
    ) -> KiRepr {
        match *condition {
            HirLazyCondition::Be { src, ref pattern } => {
                let opn = ValOpn::Be {
                    pattern_data: self.build_pattern(pattern),
                };
                let arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, src)
                )];
                val_domain_repr_guard.new_ki_repr(
                    source,
                    opn,
                    arguments,
                    self.hir_lazy_expr_control_flow_region[src],
                )
            }
            // todo: consider conversion
            HirLazyCondition::Other {
                hir_lazy_expr_idx,
                conversion,
            } => self.build_expr(val_domain_repr_guard, hir_lazy_expr_idx),
        }
    }

    fn build_expr(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        expr: HirLazyExprIdx,
    ) -> KiRepr {
        let ki_repr = self.build_expr_aux(val_domain_repr_guard, expr);
        self.hir_lazy_expr_ki_repr_map.insert_new(expr, ki_repr);
        ki_repr
    }

    fn build_expr_aux(
        &mut self,
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        expr: HirLazyExprIdx,
    ) -> KiRepr {
        let hir_lazy_expr_arena = self.hir_lazy_expr_region_data.hir_lazy_expr_arena();
        let (opn, arguments) = match hir_lazy_expr_arena[expr] {
            HirLazyExprData::Literal(lit) => (ValOpn::Literal(lit), smallvec![]),
            HirLazyExprData::PrincipalEntityPath(path) => match path {
                PrincipalEntityPath::Module(_) => todo!(),
                PrincipalEntityPath::MajorItem(path) => match path {
                    MajorItemPath::Type(_) => todo!(),
                    MajorItemPath::Trait(_) => todo!(),
                    MajorItemPath::Fugitive(path) => match path.major_fugitive_kind(self.db) {
                        MajorFugitiveKind::Ritchie(_) => todo!(),
                        MajorFugitiveKind::Const => todo!(),
                        MajorFugitiveKind::Val => return KiRepr::new_val_item(path, self.db),
                        MajorFugitiveKind::TypeAlias | MajorFugitiveKind::Formal => unreachable!(),
                    },
                },
                PrincipalEntityPath::TypeVariant(path) => (ValOpn::TypeVariant(path), smallvec![]),
            },
            HirLazyExprData::ConstSymbol(_) => todo!(),
            HirLazyExprData::Variable(var) => return self.hir_lazy_variable_ki_repr_map[var],
            HirLazyExprData::Binary { lopd, opr, ropd } => {
                let opn = ValOpn::Binary(opr);
                let arguments = smallvec![
                    KiArgumentRepr::Simple(self.build_expr(val_domain_repr_guard, lopd)),
                    KiArgumentRepr::Simple(self.build_expr(val_domain_repr_guard, ropd))
                ];
                (opn, arguments)
            }
            HirLazyExprData::Be { src, pattern: _ } => (
                ValOpn::Be {
                    pattern_data: todo!(),
                },
                smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, src)
                )],
            ),
            HirLazyExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                let opn = ValOpn::Prefix(opr);
                let arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, opd_hir_expr_idx)
                )];
                (opn, arguments)
            }
            HirLazyExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => {
                let opn = ValOpn::Suffix(opr);
                let arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, opd_hir_expr_idx)
                )];
                (opn, arguments)
            }
            HirLazyExprData::Unveil {
                opd_hir_expr_idx,
                unveil_assoc_fn_path,
                ref instantiation,
            } => {
                let opn = ValOpn::Linkage(Linkage::new_unveil_assoc_fn(
                    unveil_assoc_fn_path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                ));
                let mut arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, opd_hir_expr_idx)
                )];
                let db = self.db;
                arguments.push(KiArgumentRepr::RuntimeConstants(runtime_constants(
                    instantiation,
                    db,
                )));
                (opn, arguments)
            }
            HirLazyExprData::Unwrap { opd_hir_expr_idx } => {
                let opn = ValOpn::Unwrap {};
                let arguments = smallvec![KiArgumentRepr::Simple(
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
                let opn = ValOpn::Linkage(Linkage::new_ty_constructor_fn(
                    path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                ));
                let mut arguments: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (opn, arguments)
            }
            HirLazyExprData::TypeVariantConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::Linkage(Linkage::new_ty_variant_constructor_fn(
                    path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                ));
                let mut arguments: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (opn, arguments)
            }
            HirLazyExprData::FunctionFnItemCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::Linkage(Linkage::new_function_fn_item(
                    path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                ));
                let mut arguments: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (opn, arguments)
            }
            HirLazyExprData::AssocFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let opn = ValOpn::Linkage(Linkage::new_assoc_function_fn_item(
                    path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                ));
                let mut arguments: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (opn, arguments)
            }
            HirLazyExprData::FunctionGnItemCall {
                path,
                ref instantiation,
                ref item_groups,
                ..
            } => {
                let db = self.db;
                let Some(FugitiveHirDefn::FunctionGn(hir_defn)) = path.hir_defn(db) else {
                    unreachable!()
                };
                let opn = match hir_defn.lazy_body_with_hir_lazy_expr_region(db) {
                    Some((body, _)) => todo!(),
                    None => ValOpn::Linkage(Linkage::new_function_gn_item(
                        path,
                        instantiation,
                        &self.lin_instantiation,
                        self.db,
                    )),
                };
                let mut arguments: SmallVec<[KiArgumentRepr; 4]> = smallvec![];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (opn, arguments)
            }
            HirLazyExprData::PropsStructField {
                owner,
                owner_base_ty,
                ident,
                ..
            } => (
                ValOpn::Linkage(Linkage::new_props_struct_field(
                    owner_base_ty,
                    ident,
                    &self.lin_instantiation,
                    self.db,
                )),
                smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, owner)
                )],
            ),
            HirLazyExprData::MemoizedField {
                owner,
                path,

                ref instantiation,
                ..
            } => (
                ValOpn::Linkage(Linkage::new_memo_field(
                    path,
                    instantiation,
                    &self.lin_instantiation,
                    self.db,
                )),
                smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, owner)
                )],
            ),
            HirLazyExprData::MethodFnCall {
                self_argument,
                path,

                ref instantiation,
                ref item_groups,
                ..
            } => {
                let mut arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, self_argument)
                )];
                self.build_item_groups(
                    instantiation,
                    item_groups,
                    val_domain_repr_guard,
                    &mut arguments,
                );
                (
                    ValOpn::Linkage(Linkage::new_method(
                        path,
                        instantiation,
                        &self.lin_instantiation,
                        self.db,
                    )),
                    arguments,
                )
            }
            HirLazyExprData::NewTuple { items: _ } => todo!(),
            HirLazyExprData::Index { owner, ref items } => {
                let mut arguments = smallvec![KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, owner)
                )];
                for &item in items {
                    arguments.push(KiArgumentRepr::Simple(
                        self.build_expr(val_domain_repr_guard, item),
                    ))
                }
                // (ValOpn::Linkage(Linkage::new_index(self.db)), arguments)
                (ValOpn::Index, arguments)
            }
            HirLazyExprData::ConstructList {
                ref items,
                element_ty,
            } => (
                // todo: disambiguate between Vec, SmallVec, Array
                ValOpn::Linkage(Linkage::new_vec_constructor(
                    element_ty,
                    &self.lin_instantiation,
                    self.db,
                )),
                smallvec![KiArgumentRepr::Variadic(
                    items
                        .iter()
                        .map(|&item| { self.build_expr(val_domain_repr_guard, item) })
                        .collect()
                )],
            ),
            HirLazyExprData::Block { stmts: _ } => todo!(),
            HirLazyExprData::EmptyHtmlTag {
                function_ident: _,
                arguments: _,
            } => todo!(),
            HirLazyExprData::Todo => todo!(),
            HirLazyExprData::Unreachable => todo!(),
            HirLazyExprData::AssocFn { .. } => todo!(),
            HirLazyExprData::As { opd, ty } => todo!(),
        };
        val_domain_repr_guard.new_expr_ki_repr(
            expr,
            opn,
            arguments,
            self.hir_lazy_expr_control_flow_region[expr],
        )
    }

    // instantiation is needed for runtime constants
    fn build_item_groups(
        &mut self,
        instantiation: &HirInstantiation,
        item_groups: &[HirLazyCallListArgument],
        val_domain_repr_guard: &mut ValDomainReprGuard<'a>,
        arguments: &mut SmallVec<[KiArgumentRepr; 4]>,
    ) {
        let db = self.db;
        for item_group in item_groups {
            match *item_group {
                HirLazyCallListArgument::Simple(item) => arguments.push(KiArgumentRepr::Simple(
                    self.build_expr(val_domain_repr_guard, item),
                )),
                HirLazyCallListArgument::Variadic(ref items) => {
                    let items: SmallVec<_> = items
                        .iter()
                        .map(|&item| self.build_expr(val_domain_repr_guard, item))
                        .collect();
                    arguments.push(KiArgumentRepr::Variadic(items))
                }
                HirLazyCallListArgument::Keyed(_, item) => arguments.push(KiArgumentRepr::Keyed(
                    item.map(|item| self.build_expr(val_domain_repr_guard, item)),
                )),
            }
        }
        arguments.push(KiArgumentRepr::RuntimeConstants(runtime_constants(
            instantiation,
            db,
        )));
    }

    fn build_pattern(&mut self, pattern: &HirLazyBeVariablesPattern) -> ValPatternData {
        match pattern {
            HirLazyBeVariablesPattern::Literal => todo!(),
            HirLazyBeVariablesPattern::None => ValPatternData::None,
            HirLazyBeVariablesPattern::Some => ValPatternData::Some,
        }
    }

    fn finish(self) -> KiReprExpansion {
        KiReprExpansion::new(
            self.db,
            self.hir_lazy_variable_ki_repr_map,
            self.hir_lazy_expr_ki_repr_map,
            self.hir_lazy_stmt_ki_repr_map,
            self.root_hir_lazy_stmt_ki_reprs,
        )
    }
}

fn runtime_constants(
    instantiation: &HirInstantiation,
    db: &salsa::Db,
) -> SmallVec<[KiRuntimeConstant; 4]> {
    instantiation
        .iter()
        .filter_map(|&(symbol, res)| match symbol {
            HirTemplateSvar::Const(symbol)
                if symbol.index(db).class() == HirTemplateSvarClass::Runtime =>
            {
                Some(match res {
                    HirTermSvarResolution::Explicit(arg) => match arg {
                        HirTemplateArgument::Vacant => todo!(),
                        HirTemplateArgument::Type(_) => todo!(),
                        HirTemplateArgument::Constant(constant) => match constant {
                            HirConstant::Unit(_) => todo!(),
                            HirConstant::Bool(_) => todo!(),
                            HirConstant::Char(_) => todo!(),
                            HirConstant::I8(_) => todo!(),
                            HirConstant::I16(_) => todo!(),
                            HirConstant::I32(_) => todo!(),
                            HirConstant::I64(_) => todo!(),
                            HirConstant::I128(_) => todo!(),
                            HirConstant::ISize(_) => todo!(),
                            HirConstant::U8(_) => todo!(),
                            HirConstant::U16(_) => todo!(),
                            HirConstant::U32(_) => todo!(),
                            HirConstant::U64(_) => todo!(),
                            HirConstant::U128(_) => todo!(),
                            HirConstant::USize(_) => todo!(),
                            HirConstant::R8(_) => todo!(),
                            HirConstant::R16(_) => todo!(),
                            HirConstant::R32(_) => todo!(),
                            HirConstant::R64(_) => todo!(),
                            HirConstant::R128(_) => todo!(),
                            HirConstant::RSize(_) => todo!(),
                            HirConstant::Symbol(_) => todo!(),
                            HirConstant::TypeVariant(path) => KiRuntimeConstant::new(
                                db,
                                KiRuntimeConstantData::TypeVariantPath(path),
                            ),
                            HirConstant::StaticLifetime => todo!(),
                        },
                        HirTemplateArgument::Lifetime(_) => todo!(),
                        HirTemplateArgument::Quary(_) => todo!(),
                    },
                    HirTermSvarResolution::SelfLifetime => {
                        todo!()
                    }
                    HirTermSvarResolution::SelfPlace(_) => {
                        todo!()
                    }
                })
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
fn val_item_ki_repr_expansions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(husky_entity_path::FugitivePath, Option<KiReprExpansion>)> {
    val_item_ki_reprs(db, module_path)
        .into_iter()
        .map(|(path, ki_repr)| (path, ki_repr.expansion(db)))
        .collect()
}

#[test]
fn val_item_ki_repr_expansions_works() {
    // todo: why compiler needs this line to work?
    use husky_ast::test_utils::AstTestUtils;
    DB::ast_expect_test_debug_with_db(
        val_item_ki_repr_expansions,
        &AstTestConfig::new(
            "val_item_ki_repr_expansions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::VAL,
        ),
    )
}
