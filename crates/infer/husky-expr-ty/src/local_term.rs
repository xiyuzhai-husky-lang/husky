mod expectation;
mod implicit_conversion;
mod progress;
mod table;
mod unresolved;

pub use implicit_conversion::*;

pub(crate) use expectation::*;
pub(crate) use progress::*;
pub(crate) use table::*;
pub(crate) use unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
pub(crate) enum LocalTerm {
    Resolved(ReducedTerm),
    Unresolved(UnresolvedTermIdx),
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct LocalTermTable {
    implicit_symbol_registry: ImplicitSymbolRegistry,
    unresolved_terms: UnresolvedTerms,
    expectation_rules: LocalTermExpectations,
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
