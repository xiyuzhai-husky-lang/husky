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

pub use self::abstraction::ValidTermAbstraction;
pub use self::application::ValidTermApplication;
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
use husky_word::Identifier;
use salsa::{DebugWithDb, DisplayWithDb};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ValidTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(ValidTermLiteral),
    Symbol(ValidTermSymbol),
    EntityPath(ValidTermEntityPath),
    Category(ValidTermCategory),
    Universe(ValidTermUniverse),
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
            ValidTerm::Literal(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::Symbol(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
            ValidTerm::EntityPath(valid_term) => valid_term.show_with_db_fmt(f, db, ctx),
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
