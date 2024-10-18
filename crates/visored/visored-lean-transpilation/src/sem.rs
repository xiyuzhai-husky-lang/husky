use super::ToLean;
use lean_hir_expr::expr::LeanHirExprIdx;
use visored_sem_expr::math::VisoredSemMathExprIdx;

impl ToLean for VisoredSemMathExprIdx {
    type Target = LeanHirExprIdx;

    fn to_lean(self, builder: &mut ()) -> Self::Target {
        todo!()
    }
}
