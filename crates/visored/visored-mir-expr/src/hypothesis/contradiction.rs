use super::chunk::VdMirHypothesisChunk;

pub type VdMirContradiction = ();

pub type VdMirHypothesisResult = Result<VdMirHypothesisChunk, VdMirContradiction>;
