mod construction;
mod stack;
mod storage;

use idx_arena::ArenaIdx;

pub struct VdBaseqHypothesisData {}

pub struct VdBaseqHypothesisEntry {
    data: VdBaseqHypothesisData,
}

pub type VdBaseqHypothesisIdx = ArenaIdx<VdBaseqHypothesisEntry>;
