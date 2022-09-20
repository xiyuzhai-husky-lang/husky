use std::marker::PhantomData;

pub struct ProjApplyChangeM<This, T> {
    this: PhantomData<This>,
    cont: PhantomData<T>,
}

pub struct ProjApplyChangeR<This> {
    this: PhantomData<This>,
}

impl<This, T> std::ops::Try for ProjApplyChangeM<This, T> {
    type Output = T;

    type Residual = ProjApplyChangeR<This>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl<This, T> std::ops::FromResidual<ProjApplyChangeR<This>> for ProjApplyChangeM<This, T> {
    fn from_residual(residual: ProjApplyChangeR<This>) -> Self {
        todo!()
    }
}
