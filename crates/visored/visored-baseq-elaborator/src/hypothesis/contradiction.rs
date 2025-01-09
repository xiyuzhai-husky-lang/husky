use super::VdBsqHypothesisIdx;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdBsqHypothesisContradiction<'sess> {
    src: (),
    data: (),
    phantom: PhantomData<&'sess ()>,
}

pub type VdBsqHypothesisResult<'sess, T> = Result<T, VdBsqHypothesisContradiction<'sess>>;
