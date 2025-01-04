use crate::hypothesis::VdMirHypothesisIdx;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirCoercion {
    Trivial,
    Obvious(VdMirHypothesisIdx),
}
