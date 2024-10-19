use super::ToLean;
use lean_hir_expr::expr::LeanHirExprIdx;
use visored_sem_expr::expr::VisoredSemExprIdx;

impl ToLean for VisoredSemExprIdx {
    type Target = LeanHirExprIdx;

    fn to_lean(self, builder: &mut ()) -> Self::Target {
        todo!()
    }
}
