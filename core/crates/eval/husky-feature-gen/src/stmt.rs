mod variant;

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
                    todo!(),
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
            LazyStmtVariant::ConditionFlow { ref branches, ty } => {
                let branches: Vec<Arc<FeatureBranch>> = branches
                    .iter()
                    .map(|branch| {
                        let indicator = FeatureArrivalIndicator::new(todo!(), feature_interner);
                        Arc::new(FeatureBranch {
                            block: FeatureLazyBlock::new(
                                db,
                                opt_this.clone(),
                                &branch.stmts,
                                &symbols,
                                Some(indicator.clone()),
                                feature_interner,
                                ty,
                            ),
                            variant: match branch.variant {
                                LazyConditionBranchVariant::If { ref condition } => {
                                    FeatureBranchVariant::If {
                                        condition: FeatureExpr::new(
                                            db,
                                            opt_this.clone(),
                                            condition.clone(),
                                            &symbols,
                                            todo!(),
                                            feature_interner,
                                        ),
                                    }
                                }
                                LazyConditionBranchVariant::Elif { ref condition } => {
                                    FeatureBranchVariant::Elif {
                                        condition: FeatureExpr::new(
                                            db,
                                            opt_this.clone(),
                                            condition.clone(),
                                            &symbols,
                                            todo!(),
                                            feature_interner,
                                        ),
                                    }
                                }
                                LazyConditionBranchVariant::Else => FeatureBranchVariant::Else,
                            },
                            indicator,
                            eval_id: Default::default(),
                        })
                    })
                    .collect::<Vec<_>>();
                FeatureLazyStmtVariant::ConditionFlow { branches }
            }
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
}
