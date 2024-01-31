pub mod abstraction;
pub mod application;
pub mod application_or_ritchie_call;
pub mod associated_item;
pub mod constraint;
pub mod curry;
pub mod item_path;
pub mod list;
pub mod literal;
pub mod ritchie;
pub mod rune;
pub mod symbol;
pub mod ty_as_trai_associated_item;
pub mod wrapper;

pub use self::application::*;
pub use self::application_or_ritchie_call::*;
pub use self::associated_item::*;
pub use self::constraint::*;
pub use self::curry::*;
pub use self::item_path::*;
pub use self::list::*;
pub use self::literal::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::symbol::*;
pub use self::ty_as_trai_associated_item::*;
pub use self::wrapper::*;

use self::abstraction::*;
use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(LiteralDeclarativeTerm),
    Symbol(SymbolDeclarativeTerm),
    /// variables are those appearing in lambda expression
    /// variables are derived from symbols
    Rune(RuneDeclarativeTerm),
    EntityPath(ItemPathDeclarativeTerm),
    Category(TermCategory),
    Universe(TermUniverse),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(CurryDeclarativeTerm),
    /// in memory of Dennis M.Ritchie
    Ritchie(RitchieDeclarativeTerm),
    /// lambda x => expr
    Abstraction(AbstractionDeclarativeTerm),
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
    Application(ApplicationDeclarativeTerm),
    ApplicationOrRitchieCall(ApplicationOrRitchieCallDeclarativeTerm),
    /// ::<ident>
    AssociatedItem(AssociatedItemDeclarativeTerm),
    /// (<type> as <trait>)::<ident>
    TypeAsTraitAssociatedItem(TypeAsTraitAssociatedItemDeclarativeTerm),
    /// <type> : <trait>
    TraitConstraint(TraitConstraintDeclarativeTerm),
    /// `~`
    LeashOrBitNot(Toolchain),
    Wrapper(WrapperDeclarativeTerm),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(ListDeclarativeTerm),
}

impl salsa::DebugWithDb for DeclarativeTerm {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;
        f.write_fmt(format_args!("DeclarativeTerm(`{}`)", self.display_with(db)))
    }
}

impl salsa::DisplayWithDb for DeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DeclarativeTerm {
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> u8 {
        match self {
            DeclarativeTerm::Curry(term) => curry_parameter_count(db, term),
            _ => 0,
        }
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
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
            DeclarativeTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::ApplicationOrRitchieCall(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::AssociatedItem(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::TypeAsTraitAssociatedItem(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::LeashOrBitNot(_) => f.write_str("~"),
            DeclarativeTerm::List(term) => term.show_with_db_fmt(f, db, ctx),
            DeclarativeTerm::Wrapper(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
