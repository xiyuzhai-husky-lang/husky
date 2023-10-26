use husky_text_protocol::{HasTextRange, TextRange};
use std::sync::Arc;
use EntityPath;

use crate::{eval_id::FeatureEvalId, intern::FeatureInterner, *};
use husky_ethereal_term::EtherealTerm;

use crate::lazy_branch::ValDomainData;

use super::*;

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValBlock {
    pub region_path: RegionPath,
    pub symbols: Vec<ValSymbol>,
    pub stmts: ValStmts,
}

impl ValBlock {
    pub(crate) fn new(
        db: &dyn ValReprDb,
        opt_this: Option<ValRepr>,
        lazy_stmts: &[HirLazyStmtIdx],
        externals: &[ValSymbol],
        mut val_domain: Option<ValDomain>,
    ) -> ValBlock {
        let mut symbols: Vec<ValSymbol> = externals.into();
        // for checking
        let mut finish_flag = false;
        let mut stmts: ValStmts = smallvec![];
        for lazy_stmt in lazy_stmts {
            assert!(!finish_flag);
            let stmt = ValStmt::new_from_lazy(
                db,
                opt_this.clone(),
                lazy_stmt,
                &mut symbols,
                val_domain,
                feature_interner,
            );
            match stmt.variant {
                ValStmtData::Init { .. } | ValStmtData::Assert { .. } => (),
                ValStmtData::Return { .. } | ValStmtData::ReturnHtml { .. } => finish_flag = true,
                ValStmtData::ReturnUnveil { .. }
                | ValStmtData::Require { .. }
                | ValStmtData::ConditionFlow { .. } => {
                    val_domain = Some(ValDomain::new(
                        ValDomainData::AfterStmtNotReturn { stmt: stmt.clone() },
                        feature_interner,
                    ))
                }
            };
            stmts.push(stmt)
        }
        let feature = Feature::intern_block(feature_interner, &stmts);
        let file = stmts[0].file;
        let range = stmts.text_range();
        ValBlock::new(db, region_path, symbols, stmts)
    }

    pub(crate) fn stmt_features(&self) -> Vec<Val> {
        self.stmts
            .iter()
            .filter_map(|stmt| stmt.opt_feature)
            .collect()
    }
}
