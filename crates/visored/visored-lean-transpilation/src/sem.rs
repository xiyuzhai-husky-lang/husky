use super::ToLean;
use lean_hir_expr::expr::LnHirExprIdx;
use visored_sem_expr::expr::VdSemExprIdx;

impl ToLean for VdSemExprIdx {
    type Target = LnHirExprIdx;

    fn to_lean(self, builder: &mut ()) -> Self::Target {
        todo!()
    }
}
