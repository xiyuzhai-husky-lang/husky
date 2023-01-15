use crate::*;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermSubentity {
    parent: Term,
    ident: Identifier,
}

impl TermSubentity {
    // pub(crate) fn new(db: &dyn TermDb, parent: Term, ident: &str) -> Term {
    //     todo!()
    //     // db.it_term(
    //     //     TermSubentity {
    //     //         parent,
    //     //         ident: db.it_ident(ident),
    //     //     }
    //     //     .into(),
    //     // )
    // }
}

impl From<TermSubentity> for TermData {
    fn from(val: TermSubentity) -> Self {
        TermData::Subentity(val)
    }
}
