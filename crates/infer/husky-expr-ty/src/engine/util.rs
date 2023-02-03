use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn path(&self) -> salsa::DebugWith<'a, dyn ExprTypeDb + 'a> {
        self.expr_region_data.path_ref().debug(self.db)
    }
}
