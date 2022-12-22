mod variant;

use husky_term::Term;
pub use variant::*;

use husky_lazy_semantics::{LazyConditionBranchVariant, LazyStmt, LazyStmtVariant};
use husky_text::TextRange;
use EntityPath;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Clone)]
pub struct FeatureLazyStmt {
    pub indent: fold::Indent,
    pub variant: FeatureLazyStmtVariant,
    pub opt_arrival_indicator: Option<Arc<FeatureDomainIndicator>>,
    pub opt_feature: Option<FeatureItd>,
    pub file: DiffPath,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmt: Arc<LazyStmt>,
    pub return_ty: Term,
}

impl std::fmt::Debug for FeatureLazyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureLazyStmt")
            .field("variant", &self.variant)
            .field("file", &self.file)
            .field("range", &self.range)
            .finish()
    }
}

impl std::hash::Hash for FeatureLazyStmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureLazyStmt {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureLazyStmt {}

impl husky_text::HasTextRange for FeatureLazyStmt {
    fn text_range(&self) -> husky_text::TextRange {
        self.range
    }
}

impl FeatureLazyStmt {
    pub fn new_from_lazy(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmt: &Arc<LazyStmt>,
        symbols: &mut Vec<FeatureSymbol>,
        opt_arrival_indicator: Option<Arc<FeatureDomainIndicator>>,
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
        //         symbols.push(FeatureSymbol {
        //             varname: varname.ident,
        //             value: value.clone(),
        //             feature: value.feature,
        //         });
        //         FeatureLazyStmtVariant::Init {
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
        //         FeatureLazyStmtVariant::Assert { condition }
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
        //         FeatureLazyStmtVariant::Require {
        //             condition,
        //             return_context,
        //         }
        //     }
        //     LazyStmtVariant::Return { ref result } => FeatureLazyStmtVariant::Return {
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
        //     } => FeatureLazyStmtVariant::ReturnUnveil {
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
        //     LazyStmtVariant::ReturnXml { ref xml_expr } => FeatureLazyStmtVariant::ReturnXml {
        //         result: FeatureXmlExpr::new(
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
        // Arc::new(FeatureLazyStmt {
        //     file: lazy_stmt.file,
        //     range: lazy_stmt.range,
        //     indent: lazy_stmt.indent,
        //     opt_feature: variant.opt_feature(feature_interner),
        //     variant,
        //     eval_id: Default::default(),
        //     opt_arrival_indicator: opt_arrival_indicator.map(|s| s.clone()),
        //     stmt: lazy_stmt.clone(),
        //     return_ty: lazy_stmt.output_ty.route, // needs to instantiate this
        // })
    }

    fn new_condition_flow(
        lazy_branches: &[Arc<husky_lazy_semantics::LazyConditionBranch>],
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        symbols: &mut Vec<FeatureSymbol>,
        ty: Term,
        mut opt_arrival_indicator: Option<Arc<FeatureDomainIndicator>>,
        feature_interner: &FeatureInterner,
    ) -> FeatureLazyStmtVariant {
        let mut branches: Vec<Arc<FeatureLazyBranch>> = vec![];

        for lazy_branch in lazy_branches {
            if let Some(last_branch) = branches.last() {
                match last_branch.variant {
                    FeatureLazyBranchVariant::If { ref condition } => {
                        opt_arrival_indicator = Some(FeatureDomainIndicator::new(
                            FeatureArrivalIndicatorVariant::AfterConditionNotMet {
                                opt_parent: opt_arrival_indicator,
                                condition: condition.clone(),
                            },
                            feature_interner,
                        ));
                    }
                    FeatureLazyBranchVariant::Elif { ref condition } => {
                        opt_arrival_indicator = Some(FeatureDomainIndicator::new(
                            FeatureArrivalIndicatorVariant::AfterConditionNotMet {
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
                        Some(FeatureDomainIndicator::new(
                            FeatureArrivalIndicatorVariant::IfConditionMet {
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
                        Some(FeatureDomainIndicator::new(
                            FeatureArrivalIndicatorVariant::IfConditionMet {
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
            branches.push(Arc::new(FeatureLazyBranch {
                variant,
                opt_arrival_indicator: opt_arrival_indicator.clone(),
                eval_id: Default::default(),
                block: FeatureLazyBody::new(
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
        FeatureLazyStmtVariant::ConditionFlow { branches }
    }
}
