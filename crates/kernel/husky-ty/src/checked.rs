use husky_raw_term::RawTerm;

use crate::*;

pub struct CheckedTerm(Term);

impl CheckedTerm {
    pub fn new(db: &dyn TypeDb, term: Term) -> TypeResult<Self> {
        check_term_validity(db, term)?;
        Ok(CheckedTerm(term))
    }
}

fn check_term_validity(db: &dyn TypeDb, term: Term) -> TypeResult<()> {
    match term {
        Term::Literal(_) => Ok(()),
        Term::Symbol(term) => check_term_symbol_validity(db, term),
        Term::Placeholder(term) => check_term_placeholder_validity(db, term),
        Term::EntityPath(path) => Ok(()),
        Term::Category(_) => Ok(()),
        Term::Universe(_) => Ok(()),
        Term::Curry(term) => check_term_curry_validity(db, term),
        Term::Ritchie(term) => check_term_ritchie_validity(db, term),
        Term::Abstraction(term) => check_term_abstraction_validity(db, term),
        Term::Application(term) => check_term_application_validity(db, term),
        Term::Subentity(term) => check_term_subentity_validity(db, term),
        Term::AsTraitSubentity(term) => check_term_as_trai_subentity_validity(db, term),
        Term::TraitConstraint(term) => check_term_trai_constraint_validity(db, term),
    }
}

fn check_term_is_ins_ty0(db: &dyn TypeDb, term: Term) -> TypeResult<()> {
    check_term_validity(db, term)?;
    match term.raw_ty(db)? {
        Left(RawTerm::Category(cat)) if cat.universe().raw() == 1 => Ok(()),
        _ => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_abstraction_validity(
    db: &dyn TypeDb,
    term_abstraction: TermAbstraction,
) -> TypeResult<()> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_application_validity(
    db: &dyn TypeDb,
    term_application: TermApplication,
) -> TypeResult<()> {
    let function = term_application.function(db);
    let argument = term_application.argument(db);
    let shift = term_application.shift(db);
    check_term_validity(db, function)?;
    check_term_validity(db, argument)?;
    match shift {
        0 => {
            let function_ty = match function.ty_unchecked(db)? {
                Left(Term::Curry(function_ty)) => function_ty,
                _ => unreachable!(),
            };
            let argument_ty = argument.ty_unchecked(db)?;
            let parameter_ty = function_ty.parameter_ty(db);
            if !parameter_ty.is_ty_trivially_convertible_from(db, argument_ty)? {
                Err(OriginalTypeError::TermApplicationWrongArgumentType {
                    parameter_ty,
                    argument_ty,
                })?
            }
            Ok(())
        }
        _ => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_as_trai_subentity_validity(
    db: &dyn TypeDb,
    term_as_trai_subentity: TermAsTraitSubentity,
) -> TypeResult<()> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_trai_constraint_validity(
    db: &dyn TypeDb,
    term_trai_constraint: TermTraitConstraint,
) -> TypeResult<()> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_curry_validity(db: &dyn TypeDb, term_curry: TermCurry) -> TypeResult<()> {
    todo!()
}

pub(super) fn check_term_symbol_validity(
    db: &dyn TypeDb,
    term_symbol: TermSymbol,
) -> TypeResult<()> {
    check_term_validity(db, term_symbol.ty(db))
}

pub(super) fn check_term_placeholder_validity(
    db: &dyn TypeDb,
    term_symbol: TermPlaceholder,
) -> TypeResult<()> {
    check_term_validity(db, term_symbol.ty(db))
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_subentity_validity(
    db: &dyn TypeDb,
    term_subentity: TermSubentity,
) -> TypeResult<()> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn check_term_ritchie_validity(
    db: &dyn TypeDb,
    term_ritchie: TermRitchie,
) -> TypeResult<()> {
    for parameter_contracted_ty in term_ritchie.parameter_contracted_tys(db) {
        check_term_is_ins_ty0(db, parameter_contracted_ty.ty())?
    }
    check_term_is_ins_ty0(db, term_ritchie.return_ty(db))
}
