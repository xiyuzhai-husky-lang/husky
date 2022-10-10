use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCategoryKind {
    Type,
    Sort,
    Term,
}

impl Into<TermAtom> for TermCategoryKind {
    fn into(self) -> TermAtom {
        TermAtom {
            variant: TermAtomVariant::CategoryKind(self),
            ty_itd: None,
        }
    }
}

impl Into<Term> for TermCategoryKind {
    fn into(self) -> Term {
        Term::Atom(self.into())
    }
}
