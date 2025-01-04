pub mod number;

use crate::{
    elaborator::VdBaseqElaboratorInner,
    expr::VdMirExprFld,
    hypothesis::{
        construction::VdBaseqHypothesisConstruction, contradiction::VdBaseqHypothesisContradiction,
        VdBaseqHypothesisIdx,
    },
};
use either::*;
use visored_entity_path::{
    path::{
        set::{VdPreludeSetPath, VdSetPath},
        VdItemPath,
    },
    theorem::VdTheoremPath,
};
use visored_mir_expr::{
    coercion::VdMirCoercion, elaborator::linear::IsVdMirSequentialElaboratorInner,
    hypothesis::constructor::VdMirHypothesisConstructor,
};
use visored_term::term::VdTerm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdBaseqCoercionOutcome<'sess> {
    TriviallyTrue(VdBaseqTrivialCoercion),
    ObviouslyTrue(VdBaseqHypothesisIdx<'sess>),
    Unknown,
    Impossible(VdBaseqHypothesisContradiction<'sess>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdBaseqCoercion<'sess> {
    Trivial(VdBaseqTrivialCoercion),
    Obvious(VdBaseqHypothesisIdx<'sess>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdBaseqTrivialCoercion {
    Identity,
    NumberToReal,
}

impl<'sess> VdBaseqCoercionOutcome<'sess> {
    pub fn coercion(self) -> Option<VdBaseqCoercion<'sess>> {
        match self {
            VdBaseqCoercionOutcome::TriviallyTrue(coercion) => {
                Some(VdBaseqCoercion::Trivial(coercion))
            }
            VdBaseqCoercionOutcome::ObviouslyTrue(idx) => Some(VdBaseqCoercion::Obvious(idx)),
            VdBaseqCoercionOutcome::Unknown => None,
            VdBaseqCoercionOutcome::Impossible(_) => None,
        }
    }
}

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub(crate) fn prune_coercion(
        &mut self,
        coercion: VdBaseqCoercion<'sess>,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirCoercion {
        match coercion {
            VdBaseqCoercion::Trivial(vd_baseq_trivial_coercion) => VdMirCoercion::Trivial,
            VdBaseqCoercion::Obvious(hypothesis) => VdMirCoercion::Obvious(
                self.prune_implicit_hypothesis(hypothesis, hypothesis_constructor),
            ),
        }
    }
}
