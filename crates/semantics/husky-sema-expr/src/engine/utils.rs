use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn path(&self) -> salsa::DebugWith<'a> {
        self.syn_expr_region_data.path_ref().debug(self.db)
    }

    pub(crate) fn debug<'b>(&self, t: &'b impl salsa::DebugWithDb) -> salsa::DebugWith<'b>
    where
        'a: 'b,
    {
        t.debug(self.db())
    }
}

#[macro_use]
macro_rules! print_debug_expr {
    ($self: expr, $expr_idx: expr) => {{
        ::husky_print_utils::p!(
            $self.path(),
            $self.expr_region_data()[$expr_idx].debug($self.db())
        );
    }};
}
pub(crate) use print_debug_expr;
