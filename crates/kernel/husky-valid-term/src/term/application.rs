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
#[salsa::interned(db = RawTermDb, jar = RawTermJar, constructor = new_inner)]
pub struct RawTermApplication {
    pub function: RawTerm,
    pub argument: RawTerm,
    pub shift: u8,
}

impl RawTermApplication {
    pub fn new(
        db: &dyn RawTermDb,
        function: RawTerm,
        argument: RawTerm,
        shift: u8,
    ) -> RawTermResult<Self> {
        todo!()
    }

    pub fn from_precise(
        db: &dyn RawTermDb,
        precise_term: RawTermApplication,
    ) -> RawTermResult<Self> {
        valid_term_application_from_precise(db, precise_term)
    }

    pub(super) fn ty(self, db: &dyn RawTermDb) -> RawTermResult<RawTerm> {
        valid_term_application_ty(db, self)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn valid_term_application_from_precise(
    db: &dyn RawTermDb,
    precise_term: RawTermApplication,
) -> RawTermResult<RawTermApplication> {
    let function = RawTerm::from_precise(db, precise_term.function(db))?;
    let argument = RawTerm::from_precise(db, precise_term.argument(db))?;
    let shift = precise_term.shift(db);
    check_application_validity(db, function, argument, shift)?;
    Ok(RawTermApplication::new_inner(db, function, argument, shift))
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn valid_term_application_ty(
    db: &dyn RawTermDb,
    valid_term_application: RawTermApplication,
) -> RawTermResult<RawTerm> {
    let function = valid_term_application.function(db);
    let argument = valid_term_application.argument(db);
    let function_ty = match function.ty(db)? {
        Left(RawTerm::Curry(function_ty)) => function_ty,
        _ => return Err(todo!()),
    };
    Ok(match argument.ty(db)? {
        Left(argument_ty) => todo!(),
        Right(_) => match function_ty.parameter_symbol(db) {
            Some(function_raw_ty_parameter_symbol) => todo!(),
            None => function_ty.return_ty(db),
        },
    })
}

fn check_application_validity(
    db: &dyn RawTermDb,
    function: RawTerm,
    argument: RawTerm,
    shift: u8,
) -> RawTermResult<()> {
    match shift {
        0 => {
            let function_ty = match function.ty(db)? {
                Left(RawTerm::Curry(function_ty)) => function_ty,
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

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTermRewriteCopy for RawTermApplication {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self
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
        RawTermApplication::new(db, m, n, self.shift(db))
            .expect("substitution shouldn't return Err")
    }
}

impl std::fmt::Display for RawTermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
