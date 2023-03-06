mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod ritchie;
mod subentity;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::*;
pub use self::application::*;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_entity_path::{EntityPath, PreludeTypePath};
use husky_raw_term::RawTerm;
use husky_ty_expectation::TermTypeExpectation;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RawTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(RawTermSymbol),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(RawTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(RawTermRitchie),
    /// lambda x => expr
    Abstraction(RawTermAbstraction),

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
    Application(RawTermApplication),
    /// ::<ident>
    Subentity(RawTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(RawTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(RawTermTraitConstraint),
}

impl RawTerm {
    pub fn from_raw_unchecked(
        db: &dyn RawTermDb,
        raw_term: RawTerm,
        raw_ty_expectation: TermTypeExpectation,
    ) -> RawTermResult<Self> {
        Ok(match raw_term {
            RawTerm::Literal(literal) => {
                match literal {
                    RawTermLiteral::Resolved(literal) => literal.into(),
                    RawTermLiteral::Unresolved(_) => todo!(),
                }
                //  TermLiteral::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Symbol(raw_term) => {
                RawTermSymbol::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::EntityPath(raw_term) => match raw_term {
                RawTermEntityPath::Form(path) => TermEntityPath::Form(path).into(),
                RawTermEntityPath::Trait(path) => TermEntityPath::Trait(path).into(),
                RawTermEntityPath::Type(path) => match raw_ty_expectation {
                    TermTypeExpectation::FinalDestinationEqsSort => {
                        TermEntityPath::TypeOntology(path).into()
                    }
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(path_expected)
                        if path_expected == path =>
                    {
                        TermEntityPath::TypeConstructor(path).into()
                    }
                    TermTypeExpectation::Any => TermEntityPath::TypeConstructor(path).into(),
                    TermTypeExpectation::FinalDestinationEqsNonSortTypePath(_) => {
                        return Err(todo!())
                    }
                },
            },
            RawTerm::Category(raw_term) => raw_term.into(),
            RawTerm::Universe(raw_term) => raw_term.into(),
            RawTerm::Curry(raw_term) => {
                RawTermCurry::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Ritchie(raw_term) => {
                RawTermRitchie::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Abstraction(raw_term) => {
                RawTermAbstraction::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Application(raw_term) => {
                RawTermApplication::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Subentity(raw_term) => {
                RawTermSubentity::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::AsTraitSubentity(raw_term) => {
                RawTermAsTraitSubentity::from_raw_unchecked(db, raw_term, raw_ty_expectation)?
                    .into()
            }
            RawTerm::TraitConstraint(raw_term) => {
                RawTermTraitConstraint::from_raw_unchecked(db, raw_term, raw_ty_expectation)?.into()
            }
        })
    }

    fn raw_ty(self, db: &dyn RawTermDb) -> RawTermResult<Either<RawTerm, PreludeTypePath>> {
        Ok(match self {
            RawTerm::Literal(literal) => Right(literal.ty()),
            // term.raw_ty(db),
            RawTerm::Symbol(_) => todo!(),
            RawTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => Left(ty_ontology_path_raw_ty(db, path)?),
                TermEntityPath::TypeConstructor(path) => {
                    Left(ty_constructor_path_raw_ty(db, path)?)
                }
            },
            RawTerm::Category(_) => todo!(),
            RawTerm::Universe(_) => todo!(),
            RawTerm::Curry(_) => todo!(),
            RawTerm::Ritchie(_) => todo!(),
            RawTerm::Abstraction(_) => todo!(),
            RawTerm::Application(term) => Left(term.raw_ty(db)?),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
        })
    }

    pub(crate) fn ty_total_number_of_curry_parameters(
        self,
        db: &dyn RawTermDb,
    ) -> RawTermResult<u8> {
        Ok(match self.raw_ty(db)? {
            Left(raw_ty) => raw_ty.total_number_of_curry_parameters(db),
            Right(_) => 0,
        })
    }

    /// whether two types are trivially convertible
    pub fn is_ty_trivially_convertible_from(
        self,
        db: &dyn RawTermDb,
        other_ty: Either<Self, PreludeTypePath>,
    ) -> RawTermResult<bool> {
        match other_ty {
            Left(other_ty) if other_ty == self => Ok(true),
            Left(other_ty) => {
                todo!()
            }
            Right(other_ty) => match self {
                RawTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                    Ok(ty_path.prelude_ty_path(db)? == Some(other_ty))
                }
                _ => todo!(),
            },
        }
    }
}

#[test]
fn precise_term_size_works() {
    assert_eq!(
        std::mem::size_of::<RawTerm>(),
        2 * std::mem::size_of::<usize>()
    )
}

impl<Db: RawTermDb + ?Sized> salsa::DebugWithDb<Db> for RawTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "RawTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTerm {
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

impl RawTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        match self {
            RawTerm::Literal(literal) => literal.show_with_db_fmt(f, db),
            RawTerm::Symbol(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::EntityPath(path) => path.show_with_db_fmt(f, db),
            RawTerm::Category(precise_term) => f.write_str(&precise_term.to_string()),
            RawTerm::Universe(precise_term) => f.write_str(&precise_term.to_string()),
            RawTerm::Curry(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Ritchie(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Abstraction(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Application(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::Subentity(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::AsTraitSubentity(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            RawTerm::TraitConstraint(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        }
    }
}
