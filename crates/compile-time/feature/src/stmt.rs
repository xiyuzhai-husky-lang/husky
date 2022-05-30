use file::FilePtr;
use semantics_lazy::{LazyConditionBranchVariant, LazyStmt, LazyStmtVariant};
use text::TextRange;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureStmt {
    pub indent: fold::Indent,
    pub variant: FeatureStmtVariant,
    pub(crate) opt_feature: Option<FeaturePtr>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureStmtVariant {
    Init {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    Assert {
        condition: Arc<FeatureExpr>,
    },
    Return {
        result: Arc<FeatureExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureBranch>>,
    },
}

impl FeatureStmt {
    pub fn new_from_lazy(
        db: &dyn FeatureQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmt: &Arc<LazyStmt>,
        symbols: &mut Vec<FeatureSymbol>,
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        Arc::new(match lazy_stmt.variant {
            LazyStmtVariant::Init { varname, ref value } => {
                let value =
                    FeatureExpr::new(db, opt_this.clone(), value.clone(), &symbols, features);
                symbols.push(FeatureSymbol {
                    varname: varname.ident,
                    value: value.clone(),
                    feature: value.feature,
                });
                FeatureStmt {
                    file: lazy_stmt.file,
                    range: lazy_stmt.range,
                    indent: lazy_stmt.indent,
                    opt_feature: None,
                    variant: FeatureStmtVariant::Init {
                        varname: varname.ident,
                        value,
                    },
                    eval_id: Default::default(),
                }
            }
            LazyStmtVariant::Assert { ref condition } => {
                let condition =
                    FeatureExpr::new(db, opt_this.clone(), condition.clone(), &symbols, features);
                let feature = Some(features.alloc(Feature::Assert {
                    condition: condition.feature,
                }));
                FeatureStmt {
                    file: lazy_stmt.file,
                    range: lazy_stmt.range,
                    indent: lazy_stmt.indent,
                    opt_feature: feature,
                    variant: FeatureStmtVariant::Assert { condition },
                    eval_id: Default::default(),
                }
            }
            LazyStmtVariant::Return { ref result } => {
                let result =
                    FeatureExpr::new(db, opt_this.clone(), result.clone(), &symbols, features);
                FeatureStmt {
                    file: lazy_stmt.file,
                    range: lazy_stmt.range,
                    indent: lazy_stmt.indent,
                    opt_feature: Some(result.feature),
                    variant: FeatureStmtVariant::Return { result },
                    eval_id: Default::default(),
                }
            }
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
                                features,
                            ),
                            variant: match branch.variant {
                                LazyConditionBranchVariant::If { ref condition } => {
                                    FeatureBranchVariant::If {
                                        condition: FeatureExpr::new(
                                            db,
                                            opt_this.clone(),
                                            condition.clone(),
                                            &symbols,
                                            features,
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
                                            features,
                                        ),
                                    }
                                }
                                LazyConditionBranchVariant::Else => FeatureBranchVariant::Else,
                            },
                            eval_id: Default::default(),
                        })
                    })
                    .collect();
                let feature = Some(
                    features.alloc(Feature::Branches {
                        branches: branches
                            .iter()
                            .map(|branch| match branch.variant {
                                FeatureBranchVariant::If { ref condition } => BranchedFeature {
                                    condition: Some(condition.feature),
                                    block: branch.block.feature,
                                },
                                FeatureBranchVariant::Elif { ref condition } => BranchedFeature {
                                    condition: Some(condition.feature),
                                    block: branch.block.feature,
                                },
                                FeatureBranchVariant::Else => BranchedFeature {
                                    condition: None,
                                    block: branch.block.feature,
                                },
                            })
                            .collect(),
                    }),
                );
                FeatureStmt {
                    file: lazy_stmt.file,
                    range: lazy_stmt.range,
                    indent: lazy_stmt.indent,
                    opt_feature: feature,
                    variant: FeatureStmtVariant::ConditionFlow { branches },
                    eval_id: Default::default(),
                }
            }
            LazyStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        })
    }
}
