use std::marker::PhantomData;

pub struct TrackableApplyChangeM<This, T> {
    this: PhantomData<This>,
    cont: PhantomData<T>,
}

pub struct TrackableApplyChangeR<This> {
    this: PhantomData<This>,
}

impl<This, T> std::ops::Try for TrackableApplyChangeM<This, T> {
    type Output = T;

    type Residual = TrackableApplyChangeR<This>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl<This, T> std::ops::FromResidual<TrackableApplyChangeR<This>>
    for TrackableApplyChangeM<This, T>
{
    fn from_residual(residual: TrackableApplyChangeR<This>) -> Self {
        todo!()
    }
}
