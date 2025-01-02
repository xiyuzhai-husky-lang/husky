use super::VdBaseqHypothesisIdx;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct VdBaseqHypothesisContradiction<'sess> {
    src: (),
    data: (),
    phantom: PhantomData<&'sess ()>,
}

pub type VdBaseqHypothesisResult<'sess, T> = Result<T, VdBaseqHypothesisContradiction<'sess>>;
