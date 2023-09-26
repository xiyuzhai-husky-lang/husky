use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn path(&self) -> salsa::DebugWith<'a, dyn SemaExprDb + 'a> {
        self.expr_region_data.path_ref().debug(self.db)
    }

    pub(crate) fn debug<'b>(
        &self,
        t: &'b impl salsa::DebugWithDb<dyn SemaExprDb + 'a>,
    ) -> salsa::DebugWith<'b, dyn SemaExprDb + 'a> {
        t.debug(self.db())
    }
}

#[macro_use]
macro_rules! print_debug_expr {
    ($self: expr, $expr_idx: expr) => {{
        p!(
            $self.path(),
            $self.expr_region_data()[$expr_idx].debug($self.db())
        );
    }};
}
pub(crate) use print_debug_expr;
