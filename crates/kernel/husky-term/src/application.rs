use crate::*;

#[salsa::interned(jar = TermJar)]
pub struct TermApplication {
    pub m: Term,
    pub n: Term,
}

impl From<TermApplication> for Term {
    fn from(val: TermApplication) -> Self {
        Term::Application(val)
    }
}

impl TermApplication {
    pub fn ty_itd(&self) -> Option<Term> {
        // TODO: delete this
        None
    }
}

impl<'a> TermContext<'a> {
    pub(crate) fn sort(&self, _universe: TermUniverse) -> Term {
        todo!()
        // self.it_term(
        //     TermApplication {
        //         m: self.it_term(TermCategory::Sort.into()),
        //         n: self.it_term(universe.into()),
        //     }
        //     .into(),
        // )
    }
}

impl std::fmt::Display for TermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
