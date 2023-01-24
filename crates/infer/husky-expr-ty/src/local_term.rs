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
