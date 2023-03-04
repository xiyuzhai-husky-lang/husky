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

pub use self::abstraction::RawTermAbstraction;
pub use self::application::RawTermApplication;
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
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RawTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(RawTermLiteral),
    Symbol(RawTermSymbol),
    EntityPath(RawTermEntityPath),
    Category(RawTermCategory),
    Universe(RawTermUniverse),
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

impl<Db: RawTermDb + ?Sized> salsa::DebugWithDb<Db> for RawTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        use salsa::DisplayWithDb;
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
            RawTerm::Literal(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::EntityPath(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Category(term) => f.write_str(&term.to_string()),
            RawTerm::Universe(term) => f.write_str(&term.to_string()),
            RawTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
