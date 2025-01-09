pub mod litnum_equality;

use self::litnum_equality::*;
use super::{stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry};
use floated_sequential::db::FloaterDb;
use std::marker::PhantomData;
use visored_baseq_elaborator_macros::stashes;

#[stashes]
pub struct VdBsqHypothesisStashes<'sess> {
    litnum_equality: LitnumEqualityStash<'sess>,
}

impl<'sess> VdBsqHypothesisStashes<'sess> {
    pub(super) fn new() -> Self {
        Self {
            litnum_equality: LitnumEqualityStash::new(),
        }
    }
}

impl<'sess> VdBsqHypothesisStashes<'sess> {
    pub(super) fn add_hypothesis(
        &mut self,
        hypothesis_record: VdBsqHypothesisStackRecord<'sess>,
        hypothesis_entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) {
        self._add_hypothesis(hypothesis_record, hypothesis_entry, db);
    }
}
