mod variant;

pub use variant::*;

use file::FilePtr;
use semantics_lazy::{LazyConditionBranchVariant, LazyStmt, LazyStmtVariant};
use text::TextRange;
use vm::EvalResult;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub variant: FeatureStmtVariant,
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

impl text::TextRanged for FeatureStmt {
    fn text_range(&self) -> text::TextRange {
        self.range
    }
}

impl FeatureStmt {
    pub fn new_from_lazy(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmt: &Arc<LazyStmt>,
        symbols: &mut Vec<FeatureSymbol>,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let variant = match lazy_stmt.variant {
            LazyStmtVariant::Init { varname, ref value } => {
                let value = FeatureLazyExpr::new(
                    db,
                    opt_this.clone(),
                    value.clone(),
                    &symbols,
                    feature_interner,
                );
                symbols.push(FeatureSymbol {
                    varname: varname.ident,
                    value: value.clone(),
                    feature: value.feature,
                });
                FeatureStmtVariant::Init {
                    varname: varname.ident,
                    value,
                }
            }
            LazyStmtVariant::Assert { ref condition } => {
                let condition = FeatureLazyExpr::new(
                    db,
                    opt_this.clone(),
                    condition.clone(),
                    &symbols,
                    feature_interner,
                );
                FeatureStmtVariant::Assert { condition }
            }
            LazyStmtVariant::Return { ref result } => FeatureStmtVariant::Return {
                result: FeatureLazyExpr::new(
                    db,
                    opt_this.clone(),
                    result.clone(),
                    &symbols,
                    feature_interner,
                ),
            },
            LazyStmtVariant::ReturnXml { ref xml_expr } => FeatureStmtVariant::ReturnXml {
                result: FeatureXmlExpr::new(
                    db,
                    opt_this.clone(),
                    xml_expr.clone(),
                    &symbols,
                    feature_interner,
                ),
            },
            LazyStmtVariant::ConditionFlow { ref branches } => {
                let branches: Vec<Arc<FeatureBranch>> = branches
                    .iter()
                    .map(|branch| {
                        Arc::new(FeatureBranch {
                            block: FeatureLazyBlock::new(
                                db,
                                opt_this.clone(),
                                &branch.stmts,
                                &symbols,
                                feature_interner,
                            ),
                            variant: match branch.variant {
                                LazyConditionBranchVariant::If { ref condition } => {
                                    FeatureBranchVariant::If {
                                        condition: FeatureLazyExpr::new(
                                            db,
                                            opt_this.clone(),
                                            condition.clone(),
                                            &symbols,
                                            feature_interner,
                                        ),
                                    }
                                }
                                LazyConditionBranchVariant::Elif { ref condition } => {
                                    FeatureBranchVariant::Elif {
                                        condition: FeatureLazyExpr::new(
                                            db,
                                            opt_this.clone(),
                                            condition.clone(),
                                            &symbols,
                                            feature_interner,
                                        ),
                                    }
                                }
                                LazyConditionBranchVariant::Else => FeatureBranchVariant::Else,
                            },
                            eval_id: Default::default(),
                        })
                    })
                    .collect::<Vec<_>>();
                FeatureStmtVariant::ConditionFlow { branches }
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
        })
    }
}
