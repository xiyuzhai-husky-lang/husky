use crate::*;
use std::fmt::{Debug, Display};

/// in husky, application is generalized to include composition as a special case;
///
/// when shift is `0`, this is the normal application;
///
/// when shift is `1`, this is composition,
///
/// in general when shift is `n`, this is equavalent to
///
/// use abstraction `n` times, and then apply original argument to them,
///
/// then apply function to the result,
///
/// `\x1 ... \xn -> $function ($argument \x1 ... \xn)`
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar, constructor = new_unchecked)]
pub struct PreciseTermApplication {
    pub function: PreciseTerm,
    pub argument: PreciseTerm,
    pub shift: u8,
}

impl PreciseTermApplication {
    pub fn new(
        db: &dyn PreciseTermDb,
        function: PreciseTerm,
        argument: PreciseTerm,
        shift: u8,
    ) -> PreciseTermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl PreciseTermApplication {
    pub fn ty_itd(&self) -> Option<PreciseTerm> {
        // TODO: delete this
        None
    }
}

impl PreciseTermRewriteCopy for PreciseTermApplication {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self
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
        PreciseTermApplication::new(db, m, n, self.shift(db))
            .expect("substitution shouldn't return Err")
    }
}

impl std::fmt::Display for PreciseTermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
