use semantics::{DeclStmt, DeclStmtKind};

use crate::{stmt::FeatureStmtKind, unique_allocate::FeatureUniqueAllocator, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub stmts: Vec<FeatureStmt>,
    pub feature: FeaturePtr,
}

impl FeatureBlock {
    pub(crate) fn new(
        decl_stmts: &[DeclStmt],
        externals: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> FeatureBlock {
        let mut symbols: Vec<FeatureSymbol> = externals.into();
        let stmts: Vec<FeatureStmt> = decl_stmts
            .iter()
            .map(|decl_stmt| match decl_stmt.kind {
                DeclStmtKind::Init { varname, ref value } => {
                    let value = FeatureExpr::new(value, &symbols, features);
                    symbols.push(FeatureSymbol {
                        varname,
                        feature: value.feature,
                    });
                    FeatureStmt {
                        kind: FeatureStmtKind::Init { varname, value },
                        feature: None,
                    }
                }
                DeclStmtKind::Assert { ref condition } => {
                    let condition = FeatureExpr::new(condition, &symbols, features);
                    let feature = Some(features.alloc(Feature::Assert {
                        condition: condition.feature,
                    }));
                    FeatureStmt {
                        kind: FeatureStmtKind::Assert { condition },
                        feature,
                    }
                }
                DeclStmtKind::Return { ref result } => {
                    let result = FeatureExpr::new(result, &symbols, features);
                    FeatureStmt {
                        feature: Some(result.feature),
                        kind: FeatureStmtKind::Return { result },
                    }
                }
            })
            .collect();
        let feature = features.alloc(Feature::Block(
            stmts
                .iter()
                .filter_map(|stmt: &FeatureStmt| stmt.feature)
                .collect(),
        ));
        FeatureBlock {
            symbols,
            stmts,
            feature,
        }
    }
}
