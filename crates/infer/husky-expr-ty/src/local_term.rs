mod expectation;
mod progress;
mod region;
mod unresolved;
mod utils;

pub use expectation::*;
pub use progress::*;

pub(crate) use region::*;
pub(crate) use unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTermDb, jar = ExprTypeJar)]
pub enum LocalTerm {
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
}

impl From<TermLiteral> for LocalTerm {
    fn from(value: TermLiteral) -> Self {
        LocalTerm::Resolved(value.into())
    }
}
impl From<TermSymbol> for LocalTerm {
    fn from(value: TermSymbol) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermEntityPath> for LocalTerm {
    fn from(value: TermEntityPath) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCategory> for LocalTerm {
    fn from(value: TermCategory) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermUniverse> for LocalTerm {
    fn from(value: TermUniverse) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCurry> for LocalTerm {
    fn from(value: TermCurry) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermRitchie> for LocalTerm {
    fn from(value: TermRitchie) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAbstraction> for LocalTerm {
    fn from(value: TermAbstraction) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermApplication> for LocalTerm {
    fn from(value: TermApplication) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSubentity> for LocalTerm {
    fn from(value: TermSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAsTraitSubentity> for LocalTerm {
    fn from(value: TermAsTraitSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermTraitConstraint> for LocalTerm {
    fn from(value: TermTraitConstraint) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl LocalTerm {
    pub fn unresolved(self) -> Option<UnresolvedTermIdx> {
        match self {
            LocalTerm::Resolved(_) => None,
            LocalTerm::Unresolved(idx) => Some(idx),
        }
    }
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
    ) -> UnresolvedTermIdx {
        self.unresolved_terms
            .new_implicit_symbol(src_expr_idx, variant)
    }

    pub(crate) fn intern_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: UnresolvedTerm,
    ) -> LocalTerm {
        self.unresolved_terms
            .intern_unresolved_term(src_expr_idx, unresolved_term)
    }
}

impl LocalTerm {
    fn resolved(self) -> Option<Term> {
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

impl From<Term> for LocalTerm {
    fn from(v: Term) -> Self {
        Self::Resolved(v)
    }
}
