mod abstraction;
mod as_trai_subentity;
mod constraint;
mod curry;
mod entity_path;
mod explicit_application;
mod explicit_application_or_ritchie_call;
mod list;
mod literal;
mod placeholder;
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
pub use self::list::*;
pub use self::literal::*;
pub use self::placeholder::*;
pub use self::ritchie::*;
pub use self::subentity::*;
pub use self::symbol::*;

use crate::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DeclarativeTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, entityPath
    Literal(DeclarativeTermLiteral),
    Symbol(DeclarativeTermSymbol),
    /// variables are derived from symbols
    Hole(DeclarativeTermVariable),
    EntityPath(DeclarativeTermEntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(DeclarativeTermCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(DeclarativeTermRitchie),
    /// lambda x => expr
    Abstraction(DeclarativeTermAbstraction),
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
    ExplicitApplication(DeclarativeTermExplicitApplication),
    ExplicitApplicationOrRitchieCall(DeclarativeTermExplicitApplicationOrRitchieCall),
    /// ::<ident>
    Subentity(DeclarativeTermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(DeclarativeTermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(DeclarativeTermTraitConstraint),
    /// `~`
    LeashOrBitNot(Toolchain),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(DeclarativeTermList),
}

impl<Db: DeclarativeTermDb + ?Sized> salsa::DebugWithDb<Db> for DeclarativeTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        use salsa::DisplayWithDb;
        f.write_fmt(format_args!(
            "DeclarativeTerm(`{}`)",
            self.display_with(db, salsa::DisplayFormatLevel::root())
        ))
    }
}

impl<Db: DeclarativeTermDb + ?Sized> salsa::DisplayWithDb<Db> for DeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTerm {
    pub fn total_number_of_curry_parameters(self, db: &dyn DeclarativeTermDb) -> u8 {
        match self {
            DeclarativeTerm::Curry(term) => total_number_of_curry_parameters(db, term),
            _ => 0,
        }
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DeclarativeTerm::Literal(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Hole(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::EntityPath(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Category(term) => f.write_str(&term.to_string()),
            DeclarativeTerm::Universe(term) => f.write_str(&term.to_string()),
            DeclarativeTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::ExplicitApplication(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(term) => {
                term.show_with_db_fmt(f, db, ctx)
            }
            DeclarativeTerm::Subentity(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::AsTraitSubentity(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::LeashOrBitNot(_) => f.write_str("~"),
            DeclarativeTerm::List(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
