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
use husky_entity_path::EntityPath;
use husky_raw_term::RawTerm;
use husky_ty_expectation::TermTypeExpectation;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum PreciseTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(PreciseTermSymbol),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(PreciseTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(PreciseTermRitchie),
    /// lambda x => expr
    Abstraction(PreciseTermAbstraction),

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
    Application(PreciseTermApplication),
    /// ::<ident>
    Subentity(PreciseTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(PreciseTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(PreciseTermTraitConstraint),
}

impl PreciseTerm {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTerm,
        raw_ty_expectation: TermTypeExpectation,
    ) -> PreciseTermResult<Self> {
        Ok(match raw_term {
            RawTerm::Literal(raw_term) => {
                todo!()
                //  TermLiteral::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Symbol(raw_term) => {
                PreciseTermSymbol::from_raw(db, raw_term, raw_ty_expectation)?.into()
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
                PreciseTermCurry::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Ritchie(raw_term) => {
                PreciseTermRitchie::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Abstraction(raw_term) => {
                PreciseTermAbstraction::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Application(raw_term) => {
                PreciseTermApplication::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Subentity(raw_term) => {
                PreciseTermSubentity::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::AsTraitSubentity(raw_term) => {
                PreciseTermAsTraitSubentity::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::TraitConstraint(raw_term) => {
                PreciseTermTraitConstraint::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
        })
    }

    fn raw_ty(self, db: &dyn PreciseTermDb) -> PreciseTermResult<RawTerm> {
        match self {
            PreciseTerm::Literal(term) => todo!(),
            // term.raw_ty(db),
            PreciseTerm::Symbol(_) => todo!(),
            PreciseTerm::EntityPath(term) => todo!(),
            PreciseTerm::Category(_) => todo!(),
            PreciseTerm::Universe(_) => todo!(),
            PreciseTerm::Curry(_) => todo!(),
            PreciseTerm::Ritchie(_) => todo!(),
            PreciseTerm::Abstraction(_) => todo!(),
            PreciseTerm::Application(_) => todo!(),
            PreciseTerm::Subentity(_) => todo!(),
            PreciseTerm::AsTraitSubentity(_) => todo!(),
            PreciseTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[test]
fn precise_term_size_works() {
    assert_eq!(
        std::mem::size_of::<PreciseTerm>(),
        2 * std::mem::size_of::<usize>()
    )
}

impl<Db: PreciseTermDb + ?Sized> salsa::DebugWithDb<Db> for PreciseTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "PreciseTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTerm {
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

impl PreciseTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        todo!()
        // match self {
        //     PreciseTerm::Literal(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Symbol(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::EntityPath(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Category(precise_term) => f.write_str(&precise_term.to_string()),
        //     PreciseTerm::Universe(precise_term) => f.write_str(&precise_term.to_string()),
        //     PreciseTerm::Curry(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Ritchie(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Abstraction(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Application(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::Subentity(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        //     PreciseTerm::AsTraitSubentity(precise_term) => {
        //         precise_term.show_with_db_fmt(f, db, ctx)
        //     }
        //     PreciseTerm::TraitConstraint(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        // }
    }
}
