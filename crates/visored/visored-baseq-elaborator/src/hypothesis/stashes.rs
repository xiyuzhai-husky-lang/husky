mod litnum_equality;

use super::{stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry};
use std::marker::PhantomData;
use visored_baseq_elaborator_macros::stashes;

// #[stashes]
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

impl<'sess> VdBsqHypothesisStashes<'sess> {
    pub(super) fn add_hypothesis(
        &mut self,
        record: VdBsqHypothesisStackRecord<'sess>,
        entry: &VdBsqHypothesisEntry<'sess>,
    ) {
        // ad hoc
        // todo!()
    }
}
