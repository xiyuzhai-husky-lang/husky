use std::sync::Arc;

use file::FilePtr;
use semantics::{DeclBranchKind, DeclStmt, DeclStmtKind, EntityVersionControl};
use text::TextRange;

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
    pub file: FilePtr,
    pub range: TextRange,
}

impl FeatureBlock {
    pub(crate) fn new(
        vc: &EntityVersionControl,
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
                        let value = FeatureExpr::new(vc, value, &symbols, features);
                        symbols.push(FeatureSymbol {
                            varname,
                            value: value.clone(),
                            feature: value.feature,
                        });
                        FeatureStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature: None,
                            kind: FeatureStmtKind::Init { varname, value },
                        }
                    }
                    DeclStmtKind::Assert { ref condition } => {
                        let condition = FeatureExpr::new(vc, condition, &symbols, features);
                        let feature = Some(features.alloc(Feature::Assert {
                            condition: condition.feature,
                        }));
                        FeatureStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature,
                            kind: FeatureStmtKind::Assert { condition },
                        }
                    }
                    DeclStmtKind::Return { ref result } => {
                        let result = FeatureExpr::new(vc, result, &symbols, features);
                        FeatureStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature: Some(result.feature),
                            kind: FeatureStmtKind::Return { result },
                        }
                    }
                    DeclStmtKind::Branches { kind, ref branches } => {
                        let branches: Vec<Arc<FeatureBranch>> = branches
                            .iter()
                            .map(|branch| {
                                Arc::new(FeatureBranch {
                                    block: FeatureBlock::new(vc, &branch.stmts, &symbols, features),
                                    kind: match branch.kind {
                                        DeclBranchKind::If { ref condition } => {
                                            FeatureBranchKind::If {
                                                condition: FeatureExpr::new(
                                                    vc, condition, &symbols, features,
                                                ),
                                            }
                                        }
                                        DeclBranchKind::Elif { ref condition } => {
                                            FeatureBranchKind::Elif {
                                                condition: FeatureExpr::new(
                                                    vc, condition, &symbols, features,
                                                ),
                                            }
                                        }
                                        DeclBranchKind::Else => FeatureBranchKind::Else,
                                        DeclBranchKind::Case { ref pattern } => todo!(),
                                        DeclBranchKind::Default => todo!(),
                                    },
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
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature,
                            kind: FeatureStmtKind::Branches { kind, branches },
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
        let file = stmts[0].file;
        let range = text::group_text_range(&stmts);
        FeatureBlock {
            symbols,
            stmts,
            feature,
            file,
            range,
        }
    }
}
