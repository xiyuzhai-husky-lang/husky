pub mod construction;
pub mod constructor;
pub mod contradiction;
pub mod region;
pub mod stack;

use self::construction::VdBaseqHypothesisConstruction;
use crate::expr::VdMirExprFld;
use idx_arena::{Arena, ArenaIdx};

pub struct VdBaseqHypothesisEntry<'sess> {
    expr: VdMirExprFld<'sess>,
    construction: VdBaseqHypothesisConstruction<'sess>,
}

pub type VdBaseqHypothesisIdx<'sess> = ArenaIdx<VdBaseqHypothesisEntry<'sess>>;
pub type VdBaseqHypothesisArena<'sess> = Arena<VdBaseqHypothesisEntry<'sess>>;

impl<'sess> VdBaseqHypothesisEntry<'sess> {
    pub fn expr(&self) -> VdMirExprFld<'sess> {
        self.expr
    }

    pub fn construction(&self) -> &VdBaseqHypothesisConstruction<'sess> {
        &self.construction
    }
}
