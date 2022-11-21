use crate::*;
use husky_identifier::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermSubentity {
    parent: Term,
    ident: Identifier,
}

impl TermSubentity {
    pub(crate) fn new(db: &dyn TermDb, parent: Term, ident: &str) -> Term {
        todo!()
        // db.it_term(
        //     TermSubentity {
        //         parent,
        //         ident: db.it_ident(ident),
        //     }
        //     .into(),
        // )
    }
}

impl Into<TermData> for TermSubentity {
    fn into(self) -> TermData {
        TermData::Subentity(self)
    }
}
