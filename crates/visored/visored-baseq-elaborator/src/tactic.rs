pub mod assumption;
pub mod comm_ring;
pub mod library_search;
pub mod litnum_estimate;
pub mod litnum_reduce;
pub mod term_trivial;

use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::VdBsqExprFld,
    hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx},
    *,
};
use alt_option::AltOption;
use miracle::HasMiracleFull;

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTactic {
    Assumption,
    TermTrivial,
    LibrarySearch,
    CommRing,
    LitnumReduce,
    LitnumEstimate,
}

// Trivial tactics are not tracked
#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqTacticCall {
    LibrarySearch,
    CommRing,
    LitnumReduce,
    LitnumEstimate,
}

impl VdBsqTactic {
    pub fn run<'db, 'sess>(
        &self,
        prop: VdBsqExprFld<'sess>,
        elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
    ) -> Mhr<'sess> {
        match self {
            VdBsqTactic::Assumption => elaborator.assumption(prop),
            VdBsqTactic::TermTrivial => elaborator.term_trivial(prop),
            VdBsqTactic::LibrarySearch => elaborator.library_search(prop),
            VdBsqTactic::CommRing => elaborator.comm_ring(prop),
            VdBsqTactic::LitnumReduce => elaborator.litnum_reduce(prop),
            VdBsqTactic::LitnumEstimate => elaborator.litnum_estimate(prop),
        }
    }
}

impl VdBsqTacticCall {
    pub fn wrap<'db, 'sess, R>(self, m: impl ElabM<'db, 'sess, R>) -> impl ElabM<'db, 'sess, R>
    where
        'db: 'sess,
    {
        call::stack::with_call(self, m)
    }
}
