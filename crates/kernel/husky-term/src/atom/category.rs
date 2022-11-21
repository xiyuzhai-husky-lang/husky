use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCategory {
    Type,
    Sort,
    TermData,
}

impl Into<TermAtom> for TermCategory {
    fn into(self) -> TermAtom {
        TermAtom::Category(self)
    }
}

impl Into<TermData> for TermCategory {
    fn into(self) -> TermData {
        TermData::Atom(self.into())
    }
}
