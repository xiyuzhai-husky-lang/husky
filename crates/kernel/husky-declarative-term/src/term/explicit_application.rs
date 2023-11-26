use crate::*;
use std::fmt::Debug;

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
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermExplicitApplication {
    pub function: DeclarativeTerm,
    pub argument: DeclarativeTerm,
}

impl DeclarativeTermExplicitApplication {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for DeclarativeTermExplicitApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let db = db();
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermExplicitApplication {
    fn substitute(self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_m = self.function(db);
        let m = old_m.substitute(db, substituation);
        let old_n = self.argument(db);
        let n = old_n.substitute(db, substituation);
        if old_m == m && old_n == n {
            return self;
        }
        DeclarativeTermExplicitApplication::new(db, m, n)
    }
}

impl std::fmt::Display for DeclarativeTermExplicitApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
