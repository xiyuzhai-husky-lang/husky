mod abstraction;
mod application;
mod as_trai_subentity;
mod category;
mod constraint;
mod curry;
mod literal;
mod ritchie;
mod subentity;
mod symbol;
mod universe;

use std::fmt::{Debug, Display};

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use as_trai_subentity::*;
pub use category::*;
pub use constraint::*;
pub use curry::*;
pub use literal::*;
pub use ritchie::*;
use salsa::{DebugWithDb, DisplayWithDb};
pub use subentity::*;
pub use symbol::*;
pub use universe::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum Term {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(TermSymbol),
    Entity(EntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(TermRitchie),
    /// lambda x => expr
    Abstraction(TermAbstraction),
    /// f x, apply a function to term
    Application(TermApplication),
    /// ::<ident>
    Subentity(TermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(TermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(TermTraitConstraint),
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
            Term::Entity(term) => {
                term.display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
            }
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
