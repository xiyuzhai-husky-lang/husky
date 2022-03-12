use std::sync::Arc;

use file::FilePtr;
use semantics::{DeclBranchKind, DeclStmt, DeclStmtKind, EntityVersionControl, SemanticQueryGroup};
use text::TextRange;

use crate::{eval::FeatureEvalId, unique_allocate::FeatureUniqueAllocator, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyBlock {
    pub symbols: Vec<LazySymbol>,
    pub stmts: Vec<Arc<LazyStmt>>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
}

impl LazyBlock {
    pub(crate) fn new(
        db: &dyn SemanticQueryGroup,
        decl_stmts: &[Arc<DeclStmt>],
        externals: &[LazySymbol],
        features: &FeatureUniqueAllocator,
    ) -> LazyBlock {
        let mut symbols: Vec<LazySymbol> = externals.into();
        let stmts: Vec<Arc<LazyStmt>> = decl_stmts
            .iter()
            .map(|decl_stmt| {
                Arc::new(match decl_stmt.kind {
                    DeclStmtKind::Init { varname, ref value } => {
                        let value = LazyExpr::new(db, value, &symbols, features);
                        symbols.push(LazySymbol {
                            varname,
                            value: value.clone(),
                            feature: value.feature,
                        });
                        LazyStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature: None,
                            kind: LazyStmtKind::Init { varname, value },
                            eval_id: Default::default(),
                        }
                    }
                    DeclStmtKind::Assert { ref condition } => {
                        let condition = LazyExpr::new(db, condition, &symbols, features);
                        let feature = Some(features.alloc(Feature::Assert {
                            condition: condition.feature,
                        }));
                        LazyStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature,
                            kind: LazyStmtKind::Assert { condition },
                            eval_id: Default::default(),
                        }
                    }
                    DeclStmtKind::Return { ref result } => {
                        let result = LazyExpr::new(db, result, &symbols, features);
                        LazyStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature: Some(result.feature),
                            kind: LazyStmtKind::Return { result },
                            eval_id: Default::default(),
                        }
                    }
                    DeclStmtKind::Branches { kind, ref branches } => {
                        let branches: Vec<Arc<LazyBranch>> = branches
                            .iter()
                            .map(|branch| {
                                Arc::new(LazyBranch {
                                    block: LazyBlock::new(db, &branch.stmts, &symbols, features),
                                    kind: match branch.kind {
                                        DeclBranchKind::If { ref condition } => {
                                            LazyBranchKind::If {
                                                condition: LazyExpr::new(
                                                    db, condition, &symbols, features,
                                                ),
                                            }
                                        }
                                        DeclBranchKind::Elif { ref condition } => {
                                            LazyBranchKind::Elif {
                                                condition: LazyExpr::new(
                                                    db, condition, &symbols, features,
                                                ),
                                            }
                                        }
                                        DeclBranchKind::Else => LazyBranchKind::Else,
                                        DeclBranchKind::Case { ref pattern } => todo!(),
                                        DeclBranchKind::Default => todo!(),
                                    },
                                    eval_id: Default::default(),
                                })
                            })
                            .collect();
                        let feature = Some(
                            features.alloc(Feature::Branches {
                                branches: branches
                                    .iter()
                                    .map(|branch| match branch.kind {
                                        LazyBranchKind::If { ref condition } => BranchedFeature {
                                            condition: Some(condition.feature),
                                            block: branch.block.feature,
                                        },
                                        LazyBranchKind::Elif { ref condition } => BranchedFeature {
                                            condition: Some(condition.feature),
                                            block: branch.block.feature,
                                        },
                                        LazyBranchKind::Else => BranchedFeature {
                                            condition: None,
                                            block: branch.block.feature,
                                        },
                                    })
                                    .collect(),
                            }),
                        );
                        LazyStmt {
                            file: decl_stmt.file,
                            range: decl_stmt.range,
                            indent: decl_stmt.indent,
                            feature,
                            kind: LazyStmtKind::Branches { kind, branches },
                            eval_id: Default::default(),
                        }
                    }
                })
            })
            .collect();
        let feature = features.alloc(Feature::Block(
            stmts
                .iter()
                .filter_map(|stmt: &Arc<LazyStmt>| stmt.feature)
                .collect(),
        ));
        let file = stmts[0].file;
        let range = (&stmts).into();
        LazyBlock {
            symbols,
            stmts,
            feature,
            file,
            range,
            eval_id: Default::default(),
        }
    }
}
