mod expansion;
mod reduction;
mod utils;

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
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermApplication {
    pub function: EtherealTerm,
    pub argument: EtherealTerm,
    pub shift: u8,
}

#[test]
fn term_application_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermApplication>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermApplication {
    //// this constructor guarantees that the result is reduced and first-order valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub fn new(
        db: &dyn EtherealTermDb,
        function: EtherealTerm,
        argument: EtherealTerm,
    ) -> EtherealTermResult<EtherealTerm> {
        let (function_parameter_ty_curry_parameter_count, argument_expectation) = {
            match function.raw_ty(db)? {
                RawType::Declarative(DeclarativeTerm::Curry(function_declarative_ty)) => {
                    let parameter_ty = function_declarative_ty.parameter_ty(db);
                    (
                        parameter_ty.curry_parameter_count(db),
                        parameter_ty.ty_expectation(db)?,
                    )
                }
                _ => return Err(todo!()),
            }
        };
        let argument_ty_curry_parameter_count = argument.ty_curry_parameter_count(db)?;
        if argument_ty_curry_parameter_count < function_parameter_ty_curry_parameter_count {
            todo!()
        }
        let shift = argument_ty_curry_parameter_count - function_parameter_ty_curry_parameter_count;
        let term = Self::new_reduced(db, function, argument, shift);
        Ok(term)
    }

    /// this constructor guarantees that the result is reduced, not necessarily valid
    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub(super) fn new_reduced(
        db: &dyn EtherealTermDb,
        function: EtherealTerm,
        argument: EtherealTerm,
        shift: u8,
    ) -> EtherealTerm {
        Self::new_inner(db, function, argument, shift).reduce(db)
    }

    /// returns EtherealTerm instead of EtherealTermApplication because it might reduce to a non application term
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_term_application: DeclarativeTermExplicitApplication,
        term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<EtherealTerm> {
        // todo: implicit arguments
        ethereal_term_from_declarative_term_application(
            db,
            declarative_term_application,
            term_ty_expectation,
        )
    }

    pub(crate) fn declarative_ty(
        self,
        db: &dyn EtherealTermDb,
    ) -> EtherealTermResult<DeclarativeTerm> {
        ethereal_term_application_declarative_ty(db, self)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        self.function(db).show_with_db_fmt(f, db, ctx)?;
        f.write_str(" ")?;
        self.argument(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_from_declarative_term_application(
    db: &dyn EtherealTermDb,
    declarative_term_application: DeclarativeTermExplicitApplication,
    declarative_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    // todo: implicit arguments
    term_uncheck_from_declarative_term_application_aux(
        db,
        EtherealTerm::from_declarative(
            db,
            declarative_term_application.function(db),
            declarative_ty_expectation,
        )?,
        declarative_term_application.argument(db),
        declarative_ty_expectation,
    )
}

/// argument is `DeclarativeTerm` instead of `EtherealTerm` is because we need to read function type to get expectation for argument
///
pub(crate) fn term_uncheck_from_declarative_term_application_aux(
    db: &dyn EtherealTermDb,
    function: EtherealTerm,
    argument: DeclarativeTerm,
    declarative_ty_expectation: TermTypeExpectation,
) -> EtherealTermResult<EtherealTerm> {
    // todo: implicit arguments
    let (function_parameter_ty_curry_parameter_count, argument_expectation) = {
        match function.raw_ty(db)? {
            RawType::Declarative(DeclarativeTerm::Curry(function_ty)) => {
                let parameter_ty = function_ty.parameter_ty(db);
                (
                    parameter_ty.curry_parameter_count(db),
                    parameter_ty.ty_expectation(db)?,
                )
            }
            _ => Err(EtherealTermError::ExpectedCurryForApplicationFunctionType)?,
        }
    };
    let argument = EtherealTerm::from_declarative(db, argument, argument_expectation)?;
    let argument_ty_curry_parameter_count = argument.ty_curry_parameter_count(db)?;
    if argument_ty_curry_parameter_count < function_parameter_ty_curry_parameter_count {
        todo!()
    }
    let shift = argument_ty_curry_parameter_count - function_parameter_ty_curry_parameter_count;
    Ok(EtherealTermApplication::new_reduced(
        db, function, argument, shift,
    ))
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn parameter_ty_declarative_term_curry_to_argument_ty_expectation(
    db: &dyn EtherealTermDb,
    declarative_term_curry: DeclarativeTermCurry,
) -> TermTypeExpectation {
    todo!()
}
#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn parameter_ty_declarative_term_application_to_argument_ty_expectation(
    db: &dyn EtherealTermDb,
) -> TermTypeExpectation {
    todo!()
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn ethereal_term_application_declarative_ty(
    db: &dyn EtherealTermDb,
    term_application: EtherealTermApplication,
) -> EtherealTermResult<DeclarativeTerm> {
    let function = term_application.function(db);
    let argument = term_application.argument(db);
    match function.raw_ty(db)? {
        RawType::Declarative(DeclarativeTerm::Curry(function_ty)) => {
            match function_ty.parameter_variable(db) {
                Some(function_ty_parameter_variable) => {
                    ethereal_term_application_declarative_ty_dependent_aux(
                        db,
                        function_ty,
                        function_ty_parameter_variable,
                        argument.into_declarative(db),
                        argument.raw_ty(db)?,
                        term_application.shift(db),
                    )
                }
                None => ethereal_term_application_declarative_ty_nondependent_aux(
                    db,
                    function_ty,
                    argument.raw_ty(db)?,
                    term_application.shift(db),
                ),
            }
        }
        _ => return Err(todo!()),
    }
}

/// function_ty.parameter_variable(db) matches Some
pub(crate) fn ethereal_term_application_declarative_ty_dependent_aux(
    db: &dyn EtherealTermDb,
    function_ty: DeclarativeTermCurry,
    function_ty_parameter_variable: DeclarativeTermRune,
    argument: DeclarativeTerm,
    argument_ty: RawType,
    shift: u8,
) -> EtherealTermResult<DeclarativeTerm> {
    // for example, suppose that
    //
    // function_ty = (a: A) -> List a
    // function_ty_parameter_variable = a
    match shift {
        0 => Ok(function_ty.return_ty(db).substitute(
            db,
            &DeclarativeTermSubstitution::new(function_ty_parameter_variable, argument),
        )),
        shift => {
            // argument = arg
            // argument_ty = (b: B) -> C b -> A
            // shift = 2
            // then the type of the shifted application should be
            // (b: B) -> (c: C b) -> List (arg b c)
            // b, c are first created as ad hoc symbols
            // then converted to variables
            match argument_ty {
                RawType::Declarative(DeclarativeTerm::Curry(argument_ty)) => {
                    let new_parameter_ty = argument_ty.parameter_ty(db);
                    // shift is used as disambiguator
                    // this is possible because we expect in the recursion process
                    // shift never appears twice
                    let new_parameter_symbol =
                        unsafe { DeclarativeTermSymbol::new_ad_hoc(db, new_parameter_ty, shift) };
                    Ok(DeclarativeTermCurry::new_dependent(
                        db,
                        argument_ty.curry_kind(db),
                        argument_ty.variance(db),
                        new_parameter_symbol,
                        new_parameter_ty,
                        ethereal_term_application_declarative_ty_dependent_aux(
                            db,
                            function_ty,
                            function_ty_parameter_variable,
                            // corresponds to `arg b` in the example
                            DeclarativeTermExplicitApplication::new(
                                db,
                                argument,
                                new_parameter_symbol.into(),
                            )
                            .into(),
                            // corresponds to be `C b -> A` in the example
                            argument_ty
                                .return_ty_with_variable_substituted(
                                    db,
                                    new_parameter_symbol.into(),
                                )
                                .into(),
                            shift - 1,
                        )?,
                    )
                    .into())
                }
                _ => Err(todo!()),
            }
        }
    }
}

/// function_ty.parameter_variable(db) is None
pub(crate) fn ethereal_term_application_declarative_ty_nondependent_aux(
    db: &dyn EtherealTermDb,
    function_ty: DeclarativeTermCurry,
    argument_ty: RawType,
    shift: u8,
) -> EtherealTermResult<DeclarativeTerm> {
    debug_assert!(function_ty.parameter_variable(db).is_none());
    match shift {
        0 => Ok(function_ty.return_ty(db)),
        shift => match argument_ty {
            RawType::Declarative(DeclarativeTerm::Curry(argument_ty)) => {
                Ok(DeclarativeTermCurry::new_nondependent(
                    db,
                    argument_ty.curry_kind(db),
                    argument_ty.variance(db),
                    argument_ty.parameter_ty(db),
                    ethereal_term_application_declarative_ty_nondependent_aux(
                        db,
                        function_ty,
                        argument_ty.return_ty(db).into(),
                        shift - 1,
                    )?,
                )
                .into())
            }
            _ => Err(todo!()),
        },
    }
}

impl EtherealTerm {
    fn ty_curry_parameter_count(self, db: &dyn EtherealTermDb) -> EtherealTermResult<u8> {
        Ok(match self.raw_ty(db)? {
            RawType::Declarative(ty) => ty.curry_parameter_count(db),
            _ => 0,
        })
    }
}

impl<Db: EtherealTermDb + ?Sized> salsa::DisplayWithDb<Db> for EtherealTermApplication {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EtherealTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl EtherealTermApplication {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm
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
        // EtherealTermApplication::new(db, m, n, self.shift(db))
    }
}

impl std::fmt::Display for EtherealTermApplication {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermInstantiate for EtherealTermApplication {
    type Target = EtherealTerm;

    fn instantiate(
        self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealInstantiation,
    ) -> Self::Target {
        Self::new_reduced(
            db,
            self.function(db).instantiate(db, instantiation),
            self.argument(db).instantiate(db, instantiation),
            self.shift(db),
        )
    }
}
