pub mod construction;
pub mod constructor;
pub mod contradiction;
pub mod region;
pub mod stack;

use self::construction::VdBsqHypothesisConstruction;
use crate::{elaborator::VdBsqElaboratorInner, expr::VdMirExprFld};
use idx_arena::{Arena, ArenaIdx};
use visored_mir_expr::{
    expr::VdMirExprIdx,
    hypothesis::{
        construction::VdMirHypothesisConstruction, constructor::VdMirHypothesisConstructor,
        VdMirHypothesisIdx,
    },
};

pub struct VdBsqHypothesisEntry<'sess> {
    expr: VdMirExprFld<'sess>,
    construction: VdBsqHypothesisConstruction<'sess>,
}

pub type VdBsqHypothesisIdx<'sess> = ArenaIdx<VdBsqHypothesisEntry<'sess>>;
pub type VdBsqHypothesisArena<'sess> = Arena<VdBsqHypothesisEntry<'sess>>;

impl<'sess> VdBsqHypothesisEntry<'sess> {
    pub fn expr(&self) -> VdMirExprFld<'sess> {
        self.expr
    }

    pub fn construction(&self) -> &VdBsqHypothesisConstruction<'sess> {
        &self.construction
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn prune_implicit_hypothesis(
        &mut self,
        hypothesis: VdBsqHypothesisIdx<'sess>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        self.prune_hypothesis_aux(hypothesis, None, hypothesis_constructor)
    }

    #[inline(always)]
    pub(crate) fn prune_hypothesis_aux(
        &mut self,
        hypothesis: VdBsqHypothesisIdx<'sess>,
        explicit_goal: Option<VdMirExprIdx>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        let construction = match *self.hypothesis_constructor.arena()[hypothesis].construction() {
            VdBsqHypothesisConstruction::Sorry => VdMirHypothesisConstruction::Sorry,
            VdBsqHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion: todo!(),
            },
            VdBsqHypothesisConstruction::Phantom(phantom_data) => todo!(),
            VdBsqHypothesisConstruction::Assume => todo!(),
        };
        let goal = match explicit_goal {
            Some(goal) => goal,
            None => todo!(),
        };
        hypothesis_constructor.construct_new_hypothesis(goal, construction)
    }
}
