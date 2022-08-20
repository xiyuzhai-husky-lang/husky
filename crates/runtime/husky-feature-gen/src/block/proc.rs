use super::*;
use avec::Avec;
use husky_eager_semantics::ProcStmt;
use husky_entity_route::RangedEntityRoute;
use vm::__LinkageFp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureProcBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub ty: RangedEntityRoute,
    pub stmts: Avec<ProcStmt>,
    pub opt_linkage: Option<__LinkageFp>,
}

impl<'eval> std::hash::Hash for FeatureProcBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
