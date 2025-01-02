use super::VdMirHypothesisIdx;

pub type VdMirContradiction = ();

pub type VdMirHypothesisResult = Result<VdMirHypothesisIdx, VdMirContradiction>;
