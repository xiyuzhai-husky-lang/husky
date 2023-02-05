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
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
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
