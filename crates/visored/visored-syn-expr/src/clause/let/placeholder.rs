use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynPlaceholderResolution {
    pattern: VdSynExprIdx,
    ty: VdSynLetClausePlaceholderType,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynLetClausePlaceholderType {
    Expr(VdSynExprIdx),
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn build_let_placeholder_resolution(
        &self,
        pattern: VdSynExprIdx,
        ty: VdSynLetClausePlaceholderType,
    ) -> VdSynPlaceholderResolution {
        VdSynPlaceholderResolution { pattern, ty }
    }
}
