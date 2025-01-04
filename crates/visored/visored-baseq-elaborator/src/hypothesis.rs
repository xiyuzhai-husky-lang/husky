pub mod construction;
pub mod constructor;
pub mod contradiction;
pub mod region;
pub mod stack;

use self::construction::VdBaseqHypothesisConstruction;
use crate::{elaborator::VdBaseqElaboratorInner, expr::VdMirExprFld};
use idx_arena::{Arena, ArenaIdx};
use visored_mir_expr::{
    expr::VdMirExprIdx,
    hypothesis::{
        construction::VdMirHypothesisConstruction, constructor::VdMirHypothesisConstructor,
        VdMirHypothesisIdx,
    },
};

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

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub(crate) fn prune_implicit_hypothesis(
        &mut self,
        hypothesis: VdBaseqHypothesisIdx<'sess>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        self.prune_hypothesis_aux(hypothesis, None, hypothesis_constructor)
    }

    #[inline(always)]
    pub(crate) fn prune_hypothesis_aux(
        &mut self,
        hypothesis: VdBaseqHypothesisIdx<'sess>,
        explicit_goal: Option<VdMirExprIdx>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        let construction = match *self.hypothesis_constructor.arena()[hypothesis].construction() {
            VdBaseqHypothesisConstruction::Sorry => VdMirHypothesisConstruction::Sorry,
            VdBaseqHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion: todo!(),
            },
            VdBaseqHypothesisConstruction::Phantom(phantom_data) => todo!(),
        };
        let goal = match explicit_goal {
            Some(goal) => goal,
            None => todo!(),
        };
        hypothesis_constructor.construct_new_hypothesis(goal, construction)
    }
}
