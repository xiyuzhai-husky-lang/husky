use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn path(&self) -> salsa::DebugWith<'a, dyn ExprTypeDb + 'a> {
        self.expr_region_data.path_ref().debug(self.db)
    }
}
