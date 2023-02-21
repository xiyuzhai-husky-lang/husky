use crate::*;
use std::fmt::{Debug, Display};

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermApplication {
    pub function: Term,
    pub argument: Term,
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.function(db).display_with(db, level.next()).fmt(f)?;
        f.write_str(" ")?;
        self.argument(db).display_with(db, level.next()).fmt(f)
    }
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
        let old_m = self.function(db);
        let m = old_m.substitute_copy(db, substituation);
        let old_n = self.argument(db);
        let n = old_n.substitute_copy(db, substituation);
        if old_m == m && old_n == n {
            return self;
        }
        TermApplication::new(db, m, n)
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
