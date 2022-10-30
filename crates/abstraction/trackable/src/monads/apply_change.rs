use std::marker::PhantomData;

pub enum TrackableApplyChangeM<This, T> {
    Ok { this: PhantomData<This>, cont: T },
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
        match self {
            TrackableApplyChangeM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<This, T> std::ops::FromResidual<TrackableApplyChangeR<This>>
    for TrackableApplyChangeM<This, T>
{
    fn from_residual(residual: TrackableApplyChangeR<This>) -> Self {
        todo!()
    }
}
