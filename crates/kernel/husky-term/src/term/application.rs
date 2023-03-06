mod expansion;
mod reduction;

pub use self::expansion::*;
pub use self::reduction::*;

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
#[salsa::interned(db = TermDb, jar = TermJar, constructor = pub(crate) new_inner)]
pub struct TermApplication {
    pub function: Term,
    pub argument: Term,
    pub shift: u8,
}

impl TermApplication {
    //// this constructor guarantees that the result is valid
    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub fn new(db: &dyn TermDb, function: Term, argument: Term, shift: u8) -> TermResult<Term> {
        check_application_validity(db, function, argument, shift)?;
        Ok(Self::new_unchecked(db, function, argument, shift))
    }

    fn new_unchecked(db: &dyn TermDb, function: Term, argument: Term, shift: u8) -> Term {
        Self::new_inner(db, function, argument, shift).reduce(db)
    }

    /// returns Term instead of TermApplication because it might reduce to a non application term
    pub fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_application: RawTermApplication,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Term> {
        term_uncheck_from_raw_term_application(db, raw_term_application, term_ty_expectation)
    }

    pub(crate) fn raw_ty(self, db: &dyn TermDb) -> TermResult<RawTerm> {
        term_application_raw_ty(db, self)
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

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_uncheck_from_raw_term_application(
    db: &dyn TermDb,
    raw_term_application: RawTermApplication,
    raw_ty_expectation: TermTypeExpectation,
) -> TermResult<Term> {
    let function =
        Term::from_raw_unchecked(db, raw_term_application.function(db), raw_ty_expectation)?;
    let function_raw_ty = match function.raw_ty(db)? {
        Left(RawTerm::Curry(function_raw_ty)) => function_raw_ty,
        _ => return Err(todo!()),
    };
    let argument = Term::from_raw_unchecked(db, raw_term_application.argument(db), todo!())?;
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
    Ok(TermApplication::new_unchecked(
        db, function, argument, shift,
    ))
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn term_application_raw_ty(
    db: &dyn TermDb,
    term_application: TermApplication,
) -> TermResult<RawTerm> {
    let function = term_application.function(db);
    let argument = term_application.argument(db);
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

impl Term {
    fn ty_total_number_of_curry_parameters(self, db: &dyn TermDb) -> TermResult<u8> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => raw_ty.total_number_of_curry_parameters(db),
            Right(_) => 0,
        })
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
            let function_ty = match function.ty_unchecked(db)? {
                Left(Term::Curry(function_ty)) => function_ty,
                _ => unreachable!(),
            };
            let argument_ty = argument.ty_unchecked(db)?;
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
