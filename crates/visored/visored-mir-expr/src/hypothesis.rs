pub(crate) mod builder;
pub mod construction;
pub mod region;
pub mod stack;

use self::construction::VdMirHypothesisConstruction;
use crate::expr::VdMirExprIdx;
use idx_arena::{Arena, ArenaIdx};

pub struct VdMirHypothesisEntry {
    pub expr: VdMirExprIdx,
    pub construction: VdMirHypothesisConstruction,
}

pub type VdMirHypothesisIdx = ArenaIdx<VdMirHypothesisEntry>;
pub type VdMirHypothesisArena = Arena<VdMirHypothesisEntry>;
