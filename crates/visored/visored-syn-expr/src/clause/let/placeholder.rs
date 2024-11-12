use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynLetStmtPlaceholderResolution {
    pattern: VdSynExprIdx,
    ty: VdSynLetStmtPlaceholderType,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynLetStmtPlaceholderType {
    Expr(VdSynExprIdx),
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn build_let_placeholder_resolution(
        &self,
        pattern: VdSynExprIdx,
        ty: VdSynLetStmtPlaceholderType,
    ) -> VdSynLetStmtPlaceholderResolution {
        VdSynLetStmtPlaceholderResolution { pattern, ty }
    }
}
