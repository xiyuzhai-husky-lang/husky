use std::sync::Arc;

use file::FilePtr;
use semantics_lazy::*;
use text::{TextRange, TextRanged};

use crate::{eval::FeatureEvalId, unique_allocate::FeatureUniqueAllocator, *};

#[derive(Debug, Clone)]
pub struct FeatureBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub stmts: Vec<Arc<FeatureStmt>>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
}

impl std::hash::Hash for FeatureBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureBlock {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureBlock {}

impl FeatureBlock {
    pub(crate) fn new(
        db: &dyn FeatureQueryGroup,
        this: Option<FeatureRepr>,
        lazy_stmts: &[Arc<LazyStmt>],
        externals: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<FeatureBlock> {
        msg_once!("generics for feature block");
        let mut symbols: Vec<FeatureSymbol> = externals.into();
        let stmts: Vec<Arc<FeatureStmt>> = lazy_stmts
            .iter()
            .map(|feature_stmt| {
                Arc::new(match feature_stmt.kind {
                    LazyStmtKind::Init { varname, ref value } => {
                        let value =
                            FeatureExpr::new(db, this.clone(), value.clone(), &symbols, features);
                        symbols.push(FeatureSymbol {
                            varname: varname.ident,
                            value: value.clone(),
                            feature: value.feature,
                        });
                        FeatureStmt {
                            file: feature_stmt.file,
                            range: feature_stmt.range,
                            indent: feature_stmt.indent,
                            feature: None,
                            variant: FeatureStmtVariant::Init {
                                varname: varname.ident,
                                value,
                            },
                            eval_id: Default::default(),
                        }
                    }
                    LazyStmtKind::Assert { ref condition } => {
                        let condition = FeatureExpr::new(
                            db,
                            this.clone(),
                            condition.clone(),
                            &symbols,
                            features,
                        );
                        let feature = Some(features.alloc(Feature::Assert {
                            condition: condition.feature,
                        }));
                        FeatureStmt {
                            file: feature_stmt.file,
                            range: feature_stmt.range,
                            indent: feature_stmt.indent,
                            feature,
                            variant: FeatureStmtVariant::Assert { condition },
                            eval_id: Default::default(),
                        }
                    }
                    LazyStmtKind::Return { ref result } => {
                        let result =
                            FeatureExpr::new(db, this.clone(), result.clone(), &symbols, features);
                        FeatureStmt {
                            file: feature_stmt.file,
                            range: feature_stmt.range,
                            indent: feature_stmt.indent,
                            feature: Some(result.feature),
                            variant: FeatureStmtVariant::Return { result },
                            eval_id: Default::default(),
                        }
                    }
                    LazyStmtKind::Branches { kind, ref branches } => {
                        let branches: Vec<Arc<FeatureBranch>> = branches
                            .iter()
                            .map(|branch| {
                                Arc::new(FeatureBranch {
                                    block: FeatureBlock::new(
                                        db,
                                        this.clone(),
                                        &branch.stmts,
                                        &symbols,
                                        features,
                                    ),
                                    variant: match branch.kind {
                                        LazyBranchKind::If { ref condition } => {
                                            FeatureBranchVariant::If {
                                                condition: FeatureExpr::new(
                                                    db,
                                                    this.clone(),
                                                    condition.clone(),
                                                    &symbols,
                                                    features,
                                                ),
                                            }
                                        }
                                        LazyBranchKind::Elif { ref condition } => {
                                            FeatureBranchVariant::Elif {
                                                condition: FeatureExpr::new(
                                                    db,
                                                    this.clone(),
                                                    condition.clone(),
                                                    &symbols,
                                                    features,
                                                ),
                                            }
                                        }
                                        LazyBranchKind::Else => FeatureBranchVariant::Else,
                                        LazyBranchKind::Case { ref pattern } => todo!(),
                                        LazyBranchKind::Default => todo!(),
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
                                        FeatureBranchVariant::If { ref condition } => {
                                            BranchedFeature {
                                                condition: Some(condition.feature),
                                                block: branch.block.feature,
                                            }
                                        }
                                        FeatureBranchVariant::Elif { ref condition } => {
                                            BranchedFeature {
                                                condition: Some(condition.feature),
                                                block: branch.block.feature,
                                            }
                                        }
                                        FeatureBranchVariant::Else => BranchedFeature {
                                            condition: None,
                                            block: branch.block.feature,
                                        },
                                    })
                                    .collect(),
                            }),
                        );
                        FeatureStmt {
                            file: feature_stmt.file,
                            range: feature_stmt.range,
                            indent: feature_stmt.indent,
                            feature,
                            variant: FeatureStmtVariant::BranchGroup { kind, branches },
                            eval_id: Default::default(),
                        }
                    }
                })
            })
            .collect();
        let feature = Feature::block(features, &stmts);
        let file = stmts[0].file;
        let range = stmts.text_range();
        Arc::new(FeatureBlock {
            symbols,
            stmts,
            feature,
            file,
            range,
            eval_id: Default::default(),
        })
    }

    pub(crate) fn stmt_features(&self) -> Vec<FeaturePtr> {
        self.stmts.iter().filter_map(|stmt| stmt.feature).collect()
    }
}
