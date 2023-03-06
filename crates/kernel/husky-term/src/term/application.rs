mod expansion;

pub use self::expansion::*;

use super::*;
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
#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermApplication {
    pub function: Term,
    pub argument: Term,
    pub shift: u8,
}

impl TermApplication {
    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub fn new(db: &dyn TermDb, function: Term, argument: Term, shift: u8) -> TermResult<Term> {
        check_application_validity(db, function, argument, shift)?;
        Self::new_unchecked(db, function, argument, shift)
    }

    fn new_unchecked(
        db: &dyn TermDb,
        function: Term,
        argument: Term,
        shift: u8,
    ) -> TermResult<Term> {
        todo!()
    }

    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub fn from_raw(db: &dyn TermDb, valid_term: RawTermApplication) -> Term {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

fn check_application_validity(
    db: &dyn TermDb,
    function: Term,
    argument: Term,
    shift: u8,
) -> TermResult<()> {
    match shift {
        0 => {
            let function_ty = match function.ty(db)? {
                Left(Term::Curry(function_ty)) => function_ty,
                _ => unreachable!(),
            };
            let argument_ty = argument.ty(db)?;
            if !function_ty
                .parameter_ty(db)
                .is_ty_trivially_convertible_from(db, argument_ty)?
            {
                return Err(todo!());
            }
            Ok(())
        }
        _ => todo!(),
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_from_raw_application(db: &dyn TermDb, valid_term: RawTermApplication) -> Term {
    todo!()
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl TermApplication {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term
    where
        Self: Copy,
    {
        todo!()
        // let old_m = self.function(db);
        // let m = old_m.substitute(db, substituation);
        // let old_n = self.argument(db);
        // let n = old_n.substitute(db, substituation);
        // if old_m == m && old_n == n {
        //     return self;
        // }
        // TermApplication::new(db, m, n, self.shift(db))
    }
}

impl std::fmt::Display for TermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
