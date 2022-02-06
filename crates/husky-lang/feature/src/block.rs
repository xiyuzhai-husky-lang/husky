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
                        }
                    }
                    DeclStmtKind::Return { ref result } => {
                        let result = Arc::new(FeatureExpr::new(result, &symbols, features));
                        FeatureStmt {
                            feature: Some(result.feature),
                            kind: FeatureStmtKind::Return { result },
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
