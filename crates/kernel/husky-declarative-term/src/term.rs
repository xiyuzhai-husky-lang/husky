mod abstraction;
mod as_trai_subitem;
mod constraint;
mod curry;
mod explicit_application;
mod explicit_application_or_ritchie_call;
mod item_path;
mod list;
mod literal;
mod ritchie;
mod rune;
mod subitem;
mod symbol;
mod wrapper;

use salsa::{Database, DisplayWithDb};

pub use self::abstraction::*;
pub use self::as_trai_subitem::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::explicit_application::*;
pub use self::explicit_application_or_ritchie_call::*;
pub use self::item_path::*;
pub use self::list::*;
pub use self::literal::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::subitem::*;
pub use self::symbol::*;
pub use self::wrapper::*;

use crate::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DeclarativeTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(DeclarativeTermLiteral),
    Symbol(DeclarativeTermSymbol),
    /// variables are those appearing in lambda expression
    /// variables are derived from symbols
    Rune(DeclarativeTermRune),
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
    Subitem(DeclarativeTermSubitem),
    /// (<type> as <trait>)::<ident>
    AsTraitSubitem(DeclarativeTermAsTraitSubitem),
    /// <type> : <trait>
    TraitConstraint(DeclarativeTermTraitConstraint),
    /// `~`
    LeashOrBitNot(Toolchain),
    Wrapper(DeclarativeTermWrapper),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(DeclarativeTermList),
}

impl salsa::DebugWithDb for DeclarativeTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &dyn Database) -> std::fmt::Result {
        use salsa::DisplayWithDb;
        f.write_fmt(format_args!("DeclarativeTerm(`{}`)", self.display_with(db)))
    }
}

impl DisplayWithDb for DeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Database,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(
            f,
            db.as_jar_db_dyn::<DeclarativeTermJar>(),
            &mut Default::default(),
        )
    }
}

impl DeclarativeTerm {
    pub fn curry_parameter_count(self, db: &dyn DeclarativeTermDb) -> u8 {
        match self {
            DeclarativeTerm::Curry(term) => curry_parameter_count(db, term),
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
            DeclarativeTerm::Rune(term) => term.show_with_db_fmt(f, db, ctx),
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
            DeclarativeTerm::Subitem(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::AsTraitSubitem(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::LeashOrBitNot(_) => f.write_str("~"),
            DeclarativeTerm::List(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Wrapper(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
