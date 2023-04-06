mod abstraction;
mod as_trai_subentity;
mod constraint;
mod curry;
mod entity_path;
mod explicit_application;
mod explicit_application_or_ritchie_call;
mod hole;
mod list;
mod literal;
mod place;
mod ritchie;
mod subentity;
mod symbol;

pub use self::abstraction::*;
pub use self::as_trai_subentity::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::entity_path::*;
pub use self::explicit_application::*;
pub use self::explicit_application_or_ritchie_call::*;
pub use self::hole::*;
pub use self::list::*;
pub use self::literal::*;
pub use self::place::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum RawTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(RawTermLiteral),
    Symbol(RawTermSymbol),
    /// variables are derived from symbols
    Hole(RawTermPlaceholder),
    EntityPath(RawTermEntityPath),
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
    ExplicitApplication(RawTermExplicitApplication),
    ExplicitApplicationOrRitchieCall(RawTermExplicitApplicationOrRitchieCall),
    /// ::<ident>
    Subentity(RawTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(RawTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(RawTermTraitConstraint),
    /// `~`
    LeashOrBitNot(Toolchain),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(RawTermList),
    Place(RawTermQualifiedType),
}

impl<Db: RawTermDb + ?Sized> salsa::DebugWithDb<Db> for RawTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
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
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTerm {
    pub fn total_number_of_curry_parameters(self, db: &dyn RawTermDb) -> u8 {
        match self {
            RawTerm::Curry(term) => total_number_of_curry_parameters(db, term),
            _ => 0,
        }
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        match self {
            RawTerm::Literal(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Hole(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::EntityPath(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Category(term) => f.write_str(&term.to_string()),
            RawTerm::Universe(term) => f.write_str(&term.to_string()),
            RawTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::ExplicitApplication(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::ExplicitApplicationOrRitchieCall(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::LeashOrBitNot(_) => f.write_str("~"),
            RawTerm::List(term) => term.show_with_db_fmt(f, db, ctx),
            RawTerm::Place(_) => todo!(),
        }
    }
}
