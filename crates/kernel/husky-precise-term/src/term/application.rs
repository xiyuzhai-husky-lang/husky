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
#[salsa::interned(db = RawTermDb, jar = RawTermJar, constructor = pub(crate) new)]
pub struct RawTermApplication {
    pub function: RawTerm,
    pub argument: RawTerm,
    pub shift: u8,
}

impl RawTermApplication {
    pub fn from_raw(
        db: &dyn RawTermDb,
        raw_term: RawTermApplication,
        raw_ty_expectation: TermTypeExpectation,
    ) -> RawTermResult<Self> {
        precise_term_application_from_raw(db, raw_term, raw_ty_expectation)
    }

    pub(crate) fn raw_ty(self, db: &dyn RawTermDb) -> RawTermResult<RawTerm> {
        precise_term_application_raw_ty(db, self)
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
pub(crate) fn precise_term_application_from_raw(
    db: &dyn RawTermDb,
    raw_term_application: RawTermApplication,
    raw_ty_expectation: TermTypeExpectation,
) -> RawTermResult<RawTermApplication> {
    let function = RawTerm::from_raw(db, raw_term_application.function(db), raw_ty_expectation)?;
    let argument = RawTerm::from_raw(db, raw_term_application.argument(db), raw_ty_expectation)?;
    let function_raw_ty = match function.raw_ty(db)? {
        Left(RawTerm::Curry(function_raw_ty)) => function_raw_ty,
        _ => return Err(todo!()),
    };
    let argument_ty_total_number_of_curry_parameters =
        argument.ty_total_number_of_curry_parameters(db)?;
    let function_parameter_ty_total_number_of_curry_parameters = function_raw_ty
        .parameter_ty(db)
        .total_number_of_curry_parameters(db);
    if argument_ty_total_number_of_curry_parameters
        < function_parameter_ty_total_number_of_curry_parameters
    {
        todo!()
    }
    let shift = argument_ty_total_number_of_curry_parameters
        - function_parameter_ty_total_number_of_curry_parameters;
    Ok(RawTermApplication::new(db, function, argument, shift))
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn precise_term_application_raw_ty(
    db: &dyn RawTermDb,
    precise_term_application: RawTermApplication,
) -> RawTermResult<RawTerm> {
    let function = precise_term_application.function(db);
    let argument = precise_term_application.argument(db);
    let function_raw_ty = match function.raw_ty(db)? {
        Left(RawTerm::Curry(function_raw_ty)) => function_raw_ty,
        _ => return Err(todo!()),
    };
    Ok(match argument.raw_ty(db)? {
        Left(argument_raw_ty) => todo!(),
        Right(_) => match function_raw_ty.parameter_symbol(db) {
            Some(function_raw_ty_parameter_symbol) => todo!(),
            None => function_raw_ty.return_ty(db),
        },
    })
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
    }
}

impl std::fmt::Display for RawTermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
