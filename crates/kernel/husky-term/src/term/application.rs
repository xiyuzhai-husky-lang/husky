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

impl TermRewriteCopy for TermApplication {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self
    where
        Self: Copy,
    {
        todo!()
        // let old_parent = self.parent(db);
        // let parent = old_parent.substitute_copy(db, substituation);
        // let old_trai = self.trai(db);
        // let trai = old_trai.substitute_copy(db, substituation);
        // if old_parent == parent && old_trai == trai {
        //     return self;
        // }
        // let ident = self.ident(db);
        // TermAsTraitSubentity::new(db, parent, trai, ident)
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
