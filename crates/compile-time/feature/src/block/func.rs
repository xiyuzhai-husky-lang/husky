use super::*;
use avec::Avec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFuncBlock {
    pub symbols: Vec<FeatureSymbol>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmts: Avec<FuncStmt>,
}

impl std::hash::Hash for FeatureFuncBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
