use std::sync::Arc;

use semantics::{DeclStmt, DeclStmtKind};

use crate::{stmt::FeatureStmtKind, unique_allocate::FeatureUniqueAllocator, *};

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
                    DeclStmtKind::Branch {
                        ref conditional_blocks,
                        ref default_block,
                    } => {
                        let conditional_feature_blocks = conditional_blocks
                            .iter()
                            .map(|block| {
                                Arc::new(ConditionalFeatureBlock {
                                    condition: Arc::new(FeatureExpr::new(
                                        &block.condition,
                                        &symbols,
                                        features,
                                    )),
                                    block: FeatureBlock::new(&block.stmts, &symbols, features),
                                })
                            })
                            .collect();
                        let default_feature_block = default_block
                            .as_ref()
                            .map(|stmts| Arc::new(FeatureBlock::new(&stmts, &symbols, features)));
                        let feature = Some(features.alloc(Feature::Branch {
                            conditional_features: vec![],
                            default_feature: default_feature_block.as_ref().map(|block| {
                                block.stmts.iter().filter_map(|stmt| stmt.feature).collect()
                            }),
                        }));
                        FeatureStmt {
                            feature,
                            kind: FeatureStmtKind::Branch {
                                conditional_feature_blocks,
                                default_feature_block,
                            },
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
