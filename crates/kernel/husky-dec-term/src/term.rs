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
pub mod ty_as_trai_item;
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
pub use self::ty_as_trai_item::*;
pub use self::wrapper::*;

use self::abstraction::*;
use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(LiteralDecTerm),
    Symbol(SymbolDecTerm),
    /// variables are those appearing in lambda expression
    /// variables are derived from symbols
    Rune(RuneDecTerm),
    EntityPath(ItemPathDecTerm),
    Category(CategoryTerm),
    Universe(UniverseTerm),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(CurryDecTerm),
    /// in memory of Dennis M.Ritchie
    Ritchie(RitchieDecTerm),
    /// lambda x => expr
    Abstraction(AbstractionDecTerm),
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
    Application(ApplicationDecTerm),
    ApplicationOrRitchieCall(ApplicationOrRitchieCallDecTerm),
    /// ::<ident>
    AssociatedItem(AssociatedItemDecTerm),
    /// (<type> as <trait>)::<ident>
    TypeAsTraitItem(TypeAsTraitItemDecTerm),
    /// <type> : <trait>
    TraitConstraint(TraitConstraintDecTerm),
    /// `~`
    LeashOrBitNot(Toolchain),
    Wrapper(WrapperDecTerm),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(ListDecTerm),
}

impl salsa::DebugWithDb for DecTerm {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;
        f.write_fmt(format_args!("DecTerm(`{}`)", self.display_with(db)))
    }
}

impl salsa::DisplayWithDb for DecTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl DecTerm {
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> u8 {
        match self {
            DecTerm::Curry(term) => curry_parameter_count(db, term),
            _ => 0,
        }
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        match self {
            DecTerm::Literal(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Symbol(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Rune(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::EntityPath(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Category(term) => f.write_str(&term.to_string()),
            DecTerm::Universe(term) => f.write_str(&term.to_string()),
            DecTerm::Curry(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Ritchie(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Abstraction(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Application(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::ApplicationOrRitchieCall(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::AssociatedItem(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::TypeAsTraitItem(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::TraitConstraint(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::LeashOrBitNot(_) => f.write_str("~"),
            DecTerm::List(term) => term.show_with_db_fmt(f, db, ctx),
            DecTerm::Wrapper(term) => term.show_with_db_fmt(f, db, ctx),
        }
    }
}
