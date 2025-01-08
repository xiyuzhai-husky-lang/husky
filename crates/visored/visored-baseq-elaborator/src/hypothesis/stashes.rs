use std::marker::PhantomData;

pub struct VdBsqHypothesisStashes<'sess> {
    phantom: PhantomData<&'sess ()>,
}

impl<'sess> VdBsqHypothesisStashes<'sess> {
    pub(super) fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
