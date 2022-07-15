use super::*;
use avec::Avec;
use husky_entity_route::RangedEntityRoute;
use vm::{InstructionSheet, __Linkage, __SpecificRoutineLinkage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFuncBlock {
    pub opt_this: Option<FeatureRepr>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmts: Avec<FuncStmt>,
    pub ty: RangedEntityRoute,
    pub instruction_sheet: Arc<InstructionSheet>,
    pub opt_linkage: Option<__SpecificRoutineLinkage>,
}

impl<'eval> std::hash::Hash for FeatureFuncBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}
