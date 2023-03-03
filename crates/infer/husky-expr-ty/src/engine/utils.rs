use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn path(&self) -> salsa::DebugWith<'a, dyn ExprTypeDb + 'a> {
        self.expr_region_data.path_ref().debug(self.db)
    }

    pub(crate) fn print_debug_expr(&self, expr_idx: ExprIdx) {
        p!(
            self.path(),
            self.expr_region_data[expr_idx].debug(self.db())
        );
    }

    pub(super) fn expr_disambiguation(
        &self,
        expr_idx: ExprIdx,
    ) -> ExprTypeResultRef<ExprDisambiguation> {
        self.expr_ty_infos[expr_idx].disambiguation()
    }
}

#[macro_use]
macro_rules! print_debug_expr {
    ($self: expr, $expr_idx: expr) => {{
        p!(
            $self.path(),
            $self.expr_region_data[$expr_idx].debug($self.db())
        );
    }};
}
pub(crate) use print_debug_expr;
