mod table;
mod unresolved;

pub(crate) use table::*;
pub(crate) use unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LocalTerm {
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
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
