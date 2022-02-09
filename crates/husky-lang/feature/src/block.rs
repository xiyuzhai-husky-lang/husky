use std::sync::Arc;

use semantics::{DeclBranchKind, DeclStmt, DeclStmtKind};

use crate::{
    stmt::{FeatureBranchKind, FeatureStmtKind},
    unique_allocate::FeatureUniqueAllocator,
    *,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub stmts: Vec<Arc<FeatureStmt>>,
    pub feature: FeaturePtr,
}

impl FeatureBlock {
    pub(crate) fn new(
        decl_stmts: &[Arc<DeclStmt>],
        externals: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> FeatureBlock {
        let mut symbols: Vec<FeatureSymbol> = externals.into();
        let stmts: Vec<Arc<FeatureStmt>> = decl_stmts
            .iter()
            .map(|decl_stmt| {
                Arc::new(match decl_stmt.kind {
                    DeclStmtKind::Init { varname, ref value } => {
                        let value = Arc::new(FeatureExpr::new(value, &symbols, features));
                        symbols.push(FeatureSymbol {
                            varname,
                            value: value.clone(),
                            feature: value.feature,
                        });
                        FeatureStmt {
                            kind: FeatureStmtKind::Init { varname, value },
                            feature: None,
                            indent: decl_stmt.indent,
                        }
                    }
                    DeclStmtKind::Assert { ref condition } => {
                        let condition = Arc::new(FeatureExpr::new(condition, &symbols, features));
                        let feature = Some(features.alloc(Feature::Assert {
                            condition: condition.feature,
                        }));
                        FeatureStmt {
                            kind: FeatureStmtKind::Assert { condition },
                            feature,
                            indent: decl_stmt.indent,
                        }
                    }
                    DeclStmtKind::Return { ref result } => {
                        let result = Arc::new(FeatureExpr::new(result, &symbols, features));
                        FeatureStmt {
                            feature: Some(result.feature),
                            kind: FeatureStmtKind::Return { result },
                            indent: decl_stmt.indent,
                        }
                    }
                    DeclStmtKind::Branches { kind, ref branches } => {
                        let branches: Vec<Arc<FeatureBranch>> = branches
                            .iter()
                            .map(|branch| {
                                Arc::new(FeatureBranch {
                                    kind: match branch.kind {
                                        DeclBranchKind::If { ref condition } => {
                                            FeatureBranchKind::If {
                                                condition: Arc::new(FeatureExpr::new(
                                                    condition, &symbols, features,
                                                )),
                                            }
                                        }
                                        DeclBranchKind::Elif { ref condition } => {
                                            FeatureBranchKind::Elif {
                                                condition: Arc::new(FeatureExpr::new(
                                                    condition, &symbols, features,
                                                )),
                                            }
                                        }
                                        DeclBranchKind::Else => FeatureBranchKind::Else,
                                        DeclBranchKind::Case { ref pattern } => todo!(),
                                        DeclBranchKind::Default => todo!(),
                                    },
                                    block: FeatureBlock::new(&branch.stmts, &symbols, features),
                                })
                            })
                            .collect();
                        let feature = Some(
                            features.alloc(Feature::Branches {
                                branches: branches
                                    .iter()
                                    .map(|branch| match branch.kind {
                                        FeatureBranchKind::If { ref condition } => {
                                            BranchedFeature {
                                                condition: Some(condition.feature),
                                                block: branch.block.feature,
                                            }
                                        }
                                        FeatureBranchKind::Elif { ref condition } => {
                                            BranchedFeature {
                                                condition: Some(condition.feature),
                                                block: branch.block.feature,
                                            }
                                        }
                                        FeatureBranchKind::Else => BranchedFeature {
                                            condition: None,
                                            block: branch.block.feature,
                                        },
                                    })
                                    .collect(),
                            }),
                        );
                        FeatureStmt {
                            feature,
                            kind: FeatureStmtKind::Branches { kind, branches },
                            indent: decl_stmt.indent,
                        }
                    }
                })
            })
            .collect();
        let feature = features.alloc(Feature::Block(
            stmts
                .iter()
                .filter_map(|stmt: &Arc<FeatureStmt>| stmt.feature)
                .collect(),
        ));
        FeatureBlock {
            symbols,
            stmts,
            feature,
        }
    }
}
