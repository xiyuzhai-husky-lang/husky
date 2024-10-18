mod builder;

use visored_sem_expr::math::SemMathExprIdx;

pub trait ToLean: Copy {
    type Target;

    fn to_lean(self, builder: &mut ()) -> Self::Target;
}

impl ToLean for SemMathExprIdx {
    type Target = ();

    fn to_lean(self, builder: &mut ()) -> Self::Target {
        todo!()
    }
}
