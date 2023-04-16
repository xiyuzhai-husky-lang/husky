use husky_ethereal_term::EtherealTerm;

use crate::lazy_branch::FeatureArrivalIndicatorVariant;

use super::*;

#[derive(Debug, Clone)]
pub struct FeatureLazyBody {
    pub symbols: Vec<FeatureSymbol>,
    pub feature: FeatureItd,
    pub file: DiffPath,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub return_ty: EtherealTerm,
    pub stmts: Vec<Arc<FeatureLazyStmt>>,
}

impl<'eval> std::hash::Hash for FeatureLazyBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl<'eval> PartialEq for FeatureLazyBody {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl<'eval> Eq for FeatureLazyBody {}

impl<'eval> FeatureLazyBody {
    pub(crate) fn new(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmts: &[Arc<LazyStmt>],
        externals: &[FeatureSymbol],
        mut opt_arrival_indicator: Option<Arc<FeatureDomainIndicator>>,
        feature_interner: &FeatureInterner,
        ty: EtherealTerm,
    ) -> Arc<FeatureLazyBody> {
        let mut symbols: Vec<FeatureSymbol> = externals.into();
        // for checking
        let mut finish_flag = false;
        let mut stmts: Vec<Arc<FeatureLazyStmt>> = vec![];
        for lazy_stmt in lazy_stmts {
            assert!(!finish_flag);
            let stmt = FeatureLazyStmt::new_from_lazy(
                db,
                opt_this.clone(),
                lazy_stmt,
                &mut symbols,
                opt_arrival_indicator.clone(),
                feature_interner,
            );
            match stmt.variant {
                FeatureLazyStmtVariant::Init { .. } | FeatureLazyStmtVariant::Assert { .. } => (),
                FeatureLazyStmtVariant::Return { .. }
                | FeatureLazyStmtVariant::ReturnXml { .. } => finish_flag = true,
                FeatureLazyStmtVariant::ReturnUnveil { .. }
                | FeatureLazyStmtVariant::Require { .. }
                | FeatureLazyStmtVariant::ConditionFlow { .. } => {
                    opt_arrival_indicator = Some(FeatureDomainIndicator::new(
                        FeatureArrivalIndicatorVariant::AfterStmtNotReturn { stmt: stmt.clone() },
                        feature_interner,
                    ))
                }
            };
            stmts.push(stmt)
        }
        let feature = Feature::intern_block(feature_interner, &stmts);
        let file = stmts[0].file;
        let range = stmts.text_range();
        Arc::new(FeatureLazyBody {
            symbols,
            stmts,
            feature,
            file,
            range,
            eval_id: Default::default(),
            return_ty: ty,
        })
    }

    pub(crate) fn stmt_features(&self) -> Vec<FeatureItd> {
        self.stmts
            .iter()
            .filter_map(|stmt| stmt.opt_feature)
            .collect()
    }
}
