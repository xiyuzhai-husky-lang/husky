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

pub use self::abstraction::TermAbstraction;
pub use self::application::TermApplication;
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
pub enum Term {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(TermSymbol),
    EntityPath(TermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(TermRitchie),
    /// lambda x => expr
    Abstraction(TermAbstraction),

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
    Application(TermApplication),
    /// ::<ident>
    Subentity(TermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(TermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(TermTraitConstraint),
}

#[test]
fn term_size_works() {
    assert_eq!(
        std::mem::size_of::<Term>(),
        2 * std::mem::size_of::<usize>()
    )
}

impl<Db: TermDb + ?Sized> salsa::DebugWithDb<Db> for Term {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        f.write_fmt(format_args!(
            "Term(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for Term {
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

impl Term {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            Term::Literal(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            Term::EntityPath(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Category(term) => f.write_str(&term.to_string()),
            Term::Universe(term) => f.write_str(&term.to_string()),
            Term::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Application(term) => term.show_with_db_fmt(f, db, ctx),
            Term::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            Term::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            Term::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
