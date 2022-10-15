use crate::*;
use husky_word::Identifier;

#[derive(PartialEq, Eq, Hash)]
pub struct TermSubentity {
    parent: TermPtr,
    ident: Identifier,
}

impl TermSubentity {
    pub(crate) fn new(db: &dyn TermDb, parent: TermPtr, ident: &str) -> TermPtr {
        db.it_term(
            TermSubentity {
                parent,
                ident: db.it_ident(ident),
            }
            .into(),
        )
    }
}

impl Into<Term> for TermSubentity {
    fn into(self) -> Term {
        Term::Subentity(self)
    }
}
