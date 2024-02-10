use super::*;
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
#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct DecApplication {
    pub function: DecTerm,
    pub argument: DecTerm,
}

impl DecApplication {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        self.function(db).display_fmt_with_db_and_ctx(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).display_fmt_with_db_and_ctx(f, db, ctx)
    }
}

impl DecTermRewriteCopy for DecApplication {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_m = self.function(db);
        let m = old_m.substitute_copy(db, substitution);
        let old_n = self.argument(db);
        let n = old_n.substitute_copy(db, substitution);
        if old_m == m && old_n == n {
            return self;
        }
        DecApplication::new(db, m, n)
    }
}

impl std::fmt::Display for DecApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
