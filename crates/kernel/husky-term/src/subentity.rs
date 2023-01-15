use crate::*;
use husky_word::Identifier;

#[salsa::interned(jar = TermJar)]
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

impl From<TermSubentity> for Term {
    fn from(val: TermSubentity) -> Self {
        Term::Subentity(val)
    }
}
