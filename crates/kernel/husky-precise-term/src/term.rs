mod abstraction;
mod application;
mod as_trai_subentity;
mod category;
mod constraint;
mod curry;
mod entity_path;
mod literal;
mod ritchie;
mod subentity;
mod symbol;
mod universe;

use std::fmt::{Debug, Display};

pub use self::abstraction::PreciseTermAbstraction;
pub use self::application::PreciseTermApplication;
pub use self::as_trai_subentity::*;
pub use self::category::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::entity_path::*;
pub use self::literal::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;
pub use self::universe::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_raw_term::RawTerm;
use husky_ty_expectation::TypeExpectation;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum PreciseTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(PreciseTermLiteral),
    Symbol(PreciseTermSymbol),
    EntityPath(PreciseTermEntityPath),
    Category(PreciseTermCategory),
    Universe(PreciseTermUniverse),
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
        raw_ty_expectation: TypeExpectation,
    ) -> PreciseTermResult<Self> {
        Ok(match raw_term {
            RawTerm::Literal(raw_term) => {
                PreciseTermLiteral::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Symbol(raw_term) => {
                PreciseTermSymbol::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::EntityPath(raw_term) => {
                PreciseTermEntityPath::from_raw(db, raw_term, raw_ty_expectation)?.into()
            }
            RawTerm::Category(raw_term) => {
                PreciseTermCategory::from_raw(db, raw_term, raw_ty_expectation).into()
            }
            RawTerm::Universe(raw_term) => {
                PreciseTermUniverse::from_raw(db, raw_term, raw_ty_expectation).into()
            }
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
        match self {
            PreciseTerm::Literal(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Symbol(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::EntityPath(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Category(precise_term) => f.write_str(&precise_term.to_string()),
            PreciseTerm::Universe(precise_term) => f.write_str(&precise_term.to_string()),
            PreciseTerm::Curry(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Ritchie(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Abstraction(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Application(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::Subentity(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
            PreciseTerm::AsTraitSubentity(precise_term) => {
                precise_term.show_with_db_fmt(f, db, ctx)
            }
            PreciseTerm::TraitConstraint(precise_term) => precise_term.show_with_db_fmt(f, db, ctx),
        }
    }
}
