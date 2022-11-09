use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCategory {
    Type,
    Sort,
    Term,
}

impl Into<TermAtom> for TermCategory {
    fn into(self) -> TermAtom {
        TermAtom::Category(self)
    }
}

impl Into<TermOwned> for TermCategory {
    fn into(self) -> TermOwned {
        TermOwned::Atom(self.into())
    }
}
