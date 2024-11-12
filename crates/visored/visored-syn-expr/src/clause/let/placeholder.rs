use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynLetStmtPlaceholderResolution {}

impl<'db> VdSynExprBuilder<'db> {
    pub fn build_let_placeholder_resolution(
        &self,
        placeholder: VdSynExprIdx,
    ) -> VdSynLetStmtPlaceholderResolution {
        todo!()
    }
}
