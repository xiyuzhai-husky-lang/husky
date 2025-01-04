use super::VdBaseqHypothesisIdx;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdBaseqHypothesisContradiction<'sess> {
    src: (),
    data: (),
    phantom: PhantomData<&'sess ()>,
}

pub type VdBaseqHypothesisResult<'sess, T> = Result<T, VdBaseqHypothesisContradiction<'sess>>;
