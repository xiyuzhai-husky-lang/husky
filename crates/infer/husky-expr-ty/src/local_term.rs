mod expectation;
mod progress;
mod table;
mod unresolved;

pub use expectation::*;
pub use progress::*;

pub(crate) use table::*;
pub(crate) use unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
pub enum LocalTerm {
    Resolved(ReducedTerm),
    Unresolved(UnresolvedTermIdx),
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LocalTermTable {
    implicit_symbol_registry: ImplicitSymbolRegistry,
    unresolved_terms: UnresolvedTerms,
    expectations: LocalTermExpectations,
}

impl LocalTermTable {
    pub fn unresolved_terms(&self) -> &UnresolvedTerms {
        &self.unresolved_terms
    }

    pub fn expectations(&self) -> &LocalTermExpectations {
        &self.expectations
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
