mod abstraction;
mod application;
mod as_trai_subentity;
mod constraint;
mod curry;
mod ritchie;
mod subentity;
mod symbol;

use std::fmt::{Debug, Display};

pub use self::abstraction::ValidTermAbstraction;
pub use self::application::ValidTermApplication;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_precise_term::*;
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ValidTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(ValidTermSymbol),
    Category(TermCategory),
    EntityPath(TermEntityPath),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(ValidTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(ValidTermRitchie),
    /// lambda x => expr
    Abstraction(ValidTermAbstraction),

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
    Application(ValidTermApplication),
    /// ::<ident>
    Subentity(ValidTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(ValidTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(ValidTermTraitConstraint),
}

impl ValidTerm {
    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTerm) -> Self {
        match precise_term {
            PreciseTerm::Literal(precise_term) => precise_term.into(),
            PreciseTerm::Symbol(precise_term) => {
                ValidTermSymbol::from_precise(db, precise_term).into()
            }
            PreciseTerm::EntityPath(precise_term) => precise_term.into(),
            PreciseTerm::Category(precise_term) => precise_term.into(),
            PreciseTerm::Universe(precise_term) => precise_term.into(),
            PreciseTerm::Curry(precise_term) => {
                ValidTermCurry::from_precise(db, precise_term).into()
            }
            PreciseTerm::Ritchie(precise_term) => {
                ValidTermRitchie::from_precise(db, precise_term).into()
            }
            PreciseTerm::Abstraction(precise_term) => {
                ValidTermAbstraction::from_precise(db, precise_term).into()
            }
            PreciseTerm::Application(precise_term) => {
                ValidTermApplication::from_precise(db, precise_term).into()
            }
            PreciseTerm::Subentity(precise_term) => {
                ValidTermSubentity::from_precise(db, precise_term).into()
            }
            PreciseTerm::AsTraitSubentity(precise_term) => {
                ValidTermAsTraitSubentity::from_precise(db, precise_term).into()
            }
            PreciseTerm::TraitConstraint(precise_term) => {
                ValidTermTraitConstraint::from_precise(db, precise_term).into()
            }
        }
    }
}

#[test]
fn valid_term_size_works() {
    assert_eq!(
        std::mem::size_of::<ValidTerm>(),
        2 * std::mem::size_of::<usize>()
    )
}

impl<Db: ValidTermDb + ?Sized> salsa::DebugWithDb<Db> for ValidTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "ValidTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl ValidTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        match self {
            ValidTerm::Literal(literal) => todo!(),
            // literal.show_with_db_fmt(f, db),
            ValidTerm::Symbol(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::EntityPath(path) => todo!(),
            // path.show_with_db_fmt(f, db),
            ValidTerm::Category(valid_term) => f.write_str(&valid_term.to_string()),
            ValidTerm::Universe(valid_term) => f.write_str(&valid_term.to_string()),
            ValidTerm::Curry(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::Ritchie(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::Abstraction(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::Application(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::Subentity(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::AsTraitSubentity(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::TraitConstraint(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
        }
    }
}
