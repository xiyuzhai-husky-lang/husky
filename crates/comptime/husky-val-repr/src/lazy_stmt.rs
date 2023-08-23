mod variant;

use husky_ethereal_term::EtherealTerm;
pub use variant::*;

use husky_lazy_semantics::{HirLazyStmt, LazyConditionBranchVariant, LazyStmtVariant};
use husky_text::TextRange;
use EntityPath;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Clone)]
pub struct ValStmt {
    pub indent: fold::Indent,
    pub variant: ValStmtData,
    pub opt_arrival_indicator: Option<ValDomain>,
    pub opt_feature: Option<Val>,
    pub file: DiffPath,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmt: HirLazyStmtIdx,
    pub return_ty: HirType,
}

impl std::fmt::Debug for ValStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValStmt")
            .field("variant", &self.variant)
            .field("file", &self.file)
            .field("range", &self.range)
            .finish()
    }
}

impl std::hash::Hash for ValStmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for ValStmt {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for ValStmt {}

impl husky_text::HasTextRange for ValStmt {
    fn text_range(&self) -> husky_text::TextRange {
        self.range
    }
}

impl ValStmt {
    pub fn new_from_lazy(
        db: &dyn ValReprDb,
        opt_this: Option<ValRepr>,
        lazy_stmt: &HirLazyStmtIdx,
        symbols: &mut Vec<ValSymbol>,
        opt_arrival_indicator: Option<ValDomain>,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        todo!()
        // if lazy_stmt.range.start.line() == 36 {
        //     p!(opt_arrival_indicator);
        //     todo!()
        // }
        // let variant = match lazy_stmt.variant {
        //     LazyStmtVariant::Init { varname, ref value } => {
        //         let value = FeatureLazyExpr::new(
        //             db,
        //             opt_this.clone(),
        //             value.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         );
        //         symbols.push(ValSymbol {
        //             varname: varname.ident,
        //             value: value.clone(),
        //             feature: value.feature,
        //         });
        //         ValStmtData::Init {
        //             varname: varname.ident,
        //             value,
        //         }
        //     }
        //     LazyStmtVariant::Assert { ref condition } => {
        //         let condition = FeatureLazyExpr::new(
        //             db,
        //             opt_this.clone(),
        //             condition.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         );
        //         ValStmtData::Assert { condition }
        //     }
        //     LazyStmtVariant::Require {
        //         ref condition,
        //         return_context,
        //     } => {
        //         let condition = FeatureLazyExpr::new(
        //             db,
        //             opt_this.clone(),
        //             condition.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         );
        //         ValStmtData::Require {
        //             condition,
        //             return_context,
        //         }
        //     }
        //     LazyStmtVariant::Return { ref result } => ValStmtData::Return {
        //         result: FeatureLazyExpr::new(
        //             db,
        //             opt_this.clone(),
        //             result.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         ),
        //     },
        //     LazyStmtVariant::ReturnUnveil {
        //         ref result,
        //         implicit_conversion,
        //         return_context,
        //     } => ValStmtData::ReturnUnveil {
        //         result: FeatureLazyExpr::new(
        //             db,
        //             opt_this.clone(),
        //             result.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         ),
        //         implicit_conversion,
        //         return_context,
        //     },
        //     LazyStmtVariant::ReturnHtml { ref xml_expr } => ValStmtData::ReturnHtml {
        //         result: FeatureHtmlExpr::new(
        //             db,
        //             opt_this.clone(),
        //             xml_expr.clone(),
        //             &symbols,
        //             opt_arrival_indicator.as_ref(),
        //             feature_interner,
        //         ),
        //     },
        //     LazyStmtVariant::ConditionFlow { ref branches, ty } => Self::new_condition_flow(
        //         branches,
        //         db,
        //         opt_this,
        //         symbols,
        //         ty,
        //         opt_arrival_indicator.clone(),
        //         feature_interner,
        //     ),
        //     LazyStmtVariant::Match { .. } => todo!(),
        // };
        // Arc::new(ValStmt {
        //     file: lazy_stmt.file,
        //     range: lazy_stmt.range,
        //     indent: lazy_stmt.indent,
        //     opt_feature: variant.opt_feature(feature_interner),
        //     variant,
        //     eval_id: Default::default(),
        //     opt_arrival_indicator: opt_arrival_indicator.map(|s| s.clone()),
        //     stmt: lazy_stmt.clone(),
        //     return_ty: lazy_stmt.return_ty.route, // needs to instantiate this
        // })
    }

    fn new_condition_flow(
        lazy_branches: &[Arc<husky_lazy_semantics::LazyConditionBranch>],
        db: &dyn ValReprDb,
        opt_this: Option<ValRepr>,
        symbols: &mut Vec<ValSymbol>,
        ty: HirType,
        mut opt_arrival_indicator: Option<ValDomain>,
        feature_interner: &FeatureInterner,
    ) -> ValStmtData {
        let mut branches: Vec<Arc<ValBranch>> = vec![];
        for lazy_branch in lazy_branches {
            if let Some(last_branch) = branches.last() {
                match last_branch.variant {
                    FeatureLazyBranchVariant::If { ref condition } => {
                        opt_arrival_indicator = Some(ValDomain::new(
                            ValDomainData::AfterConditionNotMet {
                                opt_parent: opt_arrival_indicator,
                                condition: condition.clone(),
                            },
                            feature_interner,
                        ));
                    }
                    FeatureLazyBranchVariant::Elif { ref condition } => {
                        opt_arrival_indicator = Some(ValDomain::new(
                            ValDomainData::AfterConditionNotMet {
                                opt_parent: opt_arrival_indicator,
                                condition: condition.clone(),
                            },
                            feature_interner,
                        ));
                    }
                    FeatureLazyBranchVariant::Else => panic!(),
                }
            }
            let (variant, block_opt_arrival_indicator) = match lazy_branch.variant {
                LazyConditionBranchVariant::If { ref condition } => {
                    let condition = FeatureLazyExpr::new(
                        db,
                        opt_this.clone(),
                        condition.clone(),
                        &symbols,
                        opt_arrival_indicator.as_ref(),
                        feature_interner,
                    );
                    (
                        FeatureLazyBranchVariant::If {
                            condition: condition.clone(),
                        },
                        Some(ValDomain::new(
                            ValDomainData::IfConditionMet {
                                opt_parent: opt_arrival_indicator.clone(),
                                condition,
                            },
                            feature_interner,
                        )),
                    )
                }
                LazyConditionBranchVariant::Elif { ref condition } => {
                    let condition = FeatureLazyExpr::new(
                        db,
                        opt_this.clone(),
                        condition.clone(),
                        &symbols,
                        opt_arrival_indicator.as_ref(),
                        feature_interner,
                    );
                    (
                        FeatureLazyBranchVariant::If {
                            condition: condition.clone(),
                        },
                        Some(ValDomain::new(
                            ValDomainData::IfConditionMet {
                                opt_parent: opt_arrival_indicator.clone(),
                                condition,
                            },
                            feature_interner,
                        )),
                    )
                }
                LazyConditionBranchVariant::Else => (
                    FeatureLazyBranchVariant::Else,
                    opt_arrival_indicator.clone(),
                ),
            };
            branches.push(Arc::new(ValBranch {
                variant,
                opt_arrival_indicator: opt_arrival_indicator.clone(),
                eval_id: Default::default(),
                block: ValBlock::new(
                    db,
                    opt_this.clone(),
                    &lazy_branch.stmts,
                    &symbols,
                    block_opt_arrival_indicator,
                    feature_interner,
                    ty,
                ),
            }))
        }
        ValStmtData::ConditionFlow { branches }
    }
}
