use super::*;
use avec::Avec;
use semantics_eager::ProcStmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureProcBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    stmts: Avec<ProcStmt>,
}

impl std::hash::Hash for FeatureProcBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
