use super::*;

pub(crate) fn reduced_term(db: &dyn TypeDb, term: Term) -> ReducedTerm {
    // ad hoc
    ReducedTerm(term)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReducedTerm(Term);

impl ReducedTerm {
    pub fn term(&self) -> Term {
        self.0
    }
}

impl<Db: TypeDb + ?Sized> salsa::DebugWithDb<Db> for ReducedTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.0.fmt(f, db, include_all_fields)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducedTermMenu<'a> {
    term_menu: &'a TermMenu,
}

impl<'a> ReducedTermMenu<'a> {
    pub(crate) fn new(term_menu: &'a TermMenu) -> Self {
        Self { term_menu }
    }

    // MOM

    pub fn never(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.never())
    }

    pub fn unit(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.unit())
    }

    pub fn bool(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.bool())
    }

    pub fn i32(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.i32())
    }

    pub fn r32(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.r32())
    }

    pub fn list_ty(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.list_ty())
    }
}
