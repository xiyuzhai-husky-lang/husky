use super::*;

impl FluffyTerm {
    pub fn new_application(
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        function: impl Into<Self>,
        argument: impl Into<Self>,
    ) -> TermResult<Self> {
        todo!()
    }
}
