use super::*;
use std::marker::PhantomData;

pub struct VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    phantom: PhantomData<&'sess (Scheme,)>,
}

pub trait IsVdBsqHypothesisUniqueStashScheme: IsVdBsqHypothesisStashScheme {}

impl<'sess, Scheme> VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    pub(super) fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
