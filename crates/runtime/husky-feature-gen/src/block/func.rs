use super::*;
use avec::Avec;
use husky_entity_route::RangedEntityRoute;
use husky_vm::{InstructionSheet, __Linkage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFuncBody {
    pub opt_this: Option<FeatureRepr>,
    pub feature: FeaturePtr,
    pub file: FileItd,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmts: Avec<FuncStmt>,
    pub ty: RangedEntityRoute,
    pub instruction_sheet: Arc<InstructionSheet>,
    pub opt_linkage: Option<__Linkage>,
}

impl<'eval> std::hash::Hash for FeatureFuncBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
