mod expectation;
mod progress;
mod region;
mod unresolved;

pub use expectation::*;
pub use progress::*;

pub(crate) use region::*;
pub(crate) use unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
pub enum LocalTerm {
    Resolved(ReducedTerm),
    Unresolved(UnresolvedTermIdx),
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LocalTermRegion {
    unresolved_terms: UnresolvedTerms,
    expectations: LocalTermExpectations,
}

impl LocalTermRegion {
    pub fn unresolved_terms(&self) -> &UnresolvedTerms {
        &self.unresolved_terms
    }

    pub fn expectations(&self) -> &LocalTermExpectations {
        &self.expectations
    }

    pub(crate) fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> LocalTerm {
        self.unresolved_terms
            .new_implicit_symbol(src_expr_idx, variant)
    }

    pub(crate) fn intern_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        self.unresolved_terms
            .intern_unresolved_term(src_expr_idx, unresolved_term)
    }
}

impl LocalTerm {
    fn resolved(self) -> Option<ReducedTerm> {
        match self {
            LocalTerm::Resolved(term) => Some(term),
            LocalTerm::Unresolved(_) => None,
        }
    }
}

impl From<UnresolvedTermIdx> for LocalTerm {
    fn from(v: UnresolvedTermIdx) -> Self {
        Self::Unresolved(v)
    }
}

impl From<ReducedTerm> for LocalTerm {
    fn from(v: ReducedTerm) -> Self {
        Self::Resolved(v)
    }
}
