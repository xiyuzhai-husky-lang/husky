use super::{VdMirHypothesisIdx, VdMirHypothesisIdxRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdMirHypothesisChunk {
    new_hypotheses: VdMirHypothesisIdxRange,
    main_hypothesis: VdMirHypothesisIdx,
}

impl VdMirHypothesisChunk {
    pub fn new(
        new_hypotheses: VdMirHypothesisIdxRange,
        main_hypothesis: VdMirHypothesisIdx,
    ) -> Self {
        Self {
            new_hypotheses,
            main_hypothesis,
        }
    }
}

impl VdMirHypothesisChunk {
    pub fn new_hypotheses(&self) -> &VdMirHypothesisIdxRange {
        &self.new_hypotheses
    }

    pub fn main_hypothesis(&self) -> VdMirHypothesisIdx {
        self.main_hypothesis
    }
}
