use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCategory {
    Type,
    Sort,
    TermData,
}

impl From<TermCategory> for TermAtom {
    fn from(val: TermCategory) -> Self {
        TermAtom::Category(val)
    }
}

impl From<TermCategory> for Term {
    fn from(val: TermCategory) -> Self {
        Term::Atom(val.into())
    }
}
