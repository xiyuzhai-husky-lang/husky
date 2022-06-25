use vm::EvalResult;

use super::*;

#[derive(Debug, Clone)]
pub struct FeatureLazyBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmts: Vec<Arc<FeatureStmt>>,
}

impl<'eval> std::hash::Hash for FeatureLazyBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl<'eval> PartialEq for FeatureLazyBlock {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl<'eval> Eq for FeatureLazyBlock {}

impl<'eval> FeatureLazyBlock {
    pub(crate) fn new(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        lazy_stmts: &[Arc<LazyStmt>],
        externals: &[FeatureSymbol],
        features: &FeatureInterner,
    ) -> Arc<FeatureLazyBlock> {
        emsg_once!("generics for feature block");
        let mut symbols: Vec<FeatureSymbol> = externals.into();
        let stmts: Vec<Arc<FeatureStmt>> = lazy_stmts
            .iter()
            .map(|lazy_stmt| {
                FeatureStmt::new_from_lazy(db, opt_this.clone(), lazy_stmt, &mut symbols, features)
            })
            .collect();
        let feature = Feature::block(features, &stmts);
        let file = stmts[0].file;
        let range = stmts.text_range();
        Arc::new(FeatureLazyBlock {
            symbols,
            stmts,
            feature,
            file,
            range,
            eval_id: Default::default(),
        })
    }

    pub(crate) fn stmt_features(&self) -> Vec<FeaturePtr> {
        self.stmts
            .iter()
            .filter_map(|stmt| stmt.opt_feature)
            .collect()
    }
}
