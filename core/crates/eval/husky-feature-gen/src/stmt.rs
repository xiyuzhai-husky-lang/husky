mod variant;

use husky_entity_route::RangedEntityRoute;
pub use variant::*;

use husky_file::FilePtr;
use husky_lazy_semantics::{LazyConditionBranchVariant, LazyStmt, LazyStmtVariant};
use husky_text::TextRange;
use vm::__EvalResult;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub variant: FeatureLazyStmtVariant,
    pub opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
    pub opt_feature: Option<FeaturePtr>,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
}

impl std::hash::Hash for FeatureStmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureStmt {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureStmt {}

impl husky_text::TextRanged for FeatureStmt {
    fn text_range(&self) -> husky_text::TextRange {
        self.range
    }
}

impl FeatureStmt {
    pub fn new_from_lazy(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmt: &Arc<LazyStmt>,
        symbols: &mut Vec<FeatureSymbol>,
        opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let variant = match lazy_stmt.variant {
            LazyStmtVariant::Init { varname, ref value } => {
                let value = FeatureExpr::new(
                    db,
                    opt_this.clone(),
                    value.clone(),
                    &symbols,
                    opt_arrival_indicator.as_ref(),
                    feature_interner,
                );
                symbols.push(FeatureSymbol {
                    varname: varname.ident,
                    value: value.clone(),
                    feature: value.feature,
                });
                FeatureLazyStmtVariant::Init {
                    varname: varname.ident,
                    value,
                }
            }
            LazyStmtVariant::Assert { ref condition } => {
                let condition = FeatureExpr::new(
                    db,
                    opt_this.clone(),
                    condition.clone(),
                    &symbols,
                    opt_arrival_indicator.as_ref(),
                    feature_interner,
                );
                FeatureLazyStmtVariant::Assert { condition }
            }
            LazyStmtVariant::Return { ref result } => FeatureLazyStmtVariant::Return {
                result: FeatureExpr::new(
                    db,
                    opt_this.clone(),
                    result.clone(),
                    &symbols,
                    opt_arrival_indicator.as_ref(),
                    feature_interner,
                ),
            },
            LazyStmtVariant::ReturnXml { ref xml_expr } => FeatureLazyStmtVariant::ReturnXml {
                result: FeatureXmlExpr::new(
                    db,
                    opt_this.clone(),
                    xml_expr.clone(),
                    &symbols,
                    opt_arrival_indicator.as_ref(),
                    feature_interner,
                ),
            },
            LazyStmtVariant::ConditionFlow { ref branches, ty } => Self::new_condition_flow(
                branches,
                db,
                opt_this,
                symbols,
                ty,
                opt_arrival_indicator.clone(),
                feature_interner,
            ),
            LazyStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        };
        Arc::new(FeatureStmt {
            file: lazy_stmt.file,
            range: lazy_stmt.range,
            indent: lazy_stmt.indent,
            opt_feature: variant.opt_feature(feature_interner),
            variant,
            eval_id: Default::default(),
            opt_arrival_indicator: opt_arrival_indicator.map(|s| s.clone()),
        })
    }

    fn new_condition_flow(
        lazy_branches: &[Arc<husky_lazy_semantics::LazyConditionBranch>],
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        symbols: &mut Vec<FeatureSymbol>,
        ty: RangedEntityRoute,
        mut opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
        feature_interner: &interner::Interner<Feature>,
    ) -> FeatureLazyStmtVariant {
        let mut branches: Vec<Arc<FeatureBranch>> = vec![];

        for lazy_branch in lazy_branches {
            if let Some(last_branch) = branches.last() {
                match last_branch.variant {
                    FeatureBranchVariant::If { ref condition } => {
                        opt_arrival_indicator = Some(FeatureArrivalIndicator::new(
                            FeatureBranchIndicatorVariant::AfterConditionNotMet {
                                opt_parent: opt_arrival_indicator,
                                condition: condition.clone(),
                            },
                            feature_interner,
                        ));
                    }
                    FeatureBranchVariant::Elif { ref condition } => {
                        opt_arrival_indicator = Some(FeatureArrivalIndicator::new(
                            FeatureBranchIndicatorVariant::AfterConditionNotMet {
                                opt_parent: opt_arrival_indicator,
                                condition: condition.clone(),
                            },
                            feature_interner,
                        ));
                    }
                    FeatureBranchVariant::Else => panic!(),
                }
            }
            let (variant, block_opt_arrival_indicator) = match lazy_branch.variant {
                LazyConditionBranchVariant::If { ref condition } => {
                    let condition = FeatureExpr::new(
                        db,
                        opt_this.clone(),
                        condition.clone(),
                        &symbols,
                        opt_arrival_indicator.as_ref(),
                        feature_interner,
                    );
                    (
                        FeatureBranchVariant::If {
                            condition: condition.clone(),
                        },
                        Some(FeatureArrivalIndicator::new(
                            FeatureBranchIndicatorVariant::IfConditionMet {
                                opt_parent: opt_arrival_indicator.clone(),
                                condition,
                            },
                            feature_interner,
                        )),
                    )
                }
                LazyConditionBranchVariant::Elif { ref condition } => {
                    let condition = FeatureExpr::new(
                        db,
                        opt_this.clone(),
                        condition.clone(),
                        &symbols,
                        opt_arrival_indicator.as_ref(),
                        feature_interner,
                    );
                    (
                        FeatureBranchVariant::If {
                            condition: condition.clone(),
                        },
                        Some(FeatureArrivalIndicator::new(
                            FeatureBranchIndicatorVariant::IfConditionMet {
                                opt_parent: opt_arrival_indicator.clone(),
                                condition,
                            },
                            feature_interner,
                        )),
                    )
                }
                LazyConditionBranchVariant::Else => {
                    (FeatureBranchVariant::Else, opt_arrival_indicator.clone())
                }
            };
            branches.push(Arc::new(FeatureBranch {
                variant,
                opt_arrival_indicator: opt_arrival_indicator.clone(),
                eval_id: Default::default(),
                block: FeatureLazyBlock::new(
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
