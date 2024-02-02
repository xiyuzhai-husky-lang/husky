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
use self::name::SymbolDecTermNameMap;
use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecTerm {
    /// atoms
    ///
    /// literal: 1,1.0, true, false; variable, itemPath
    Literal(DecLiteral),
    Symbol(DecSymbol),
    /// variables are those appearing in lambda expression
    /// variables are derived from symbols
    Rune(DecRune),
    EntityPath(DecItemPath),
    Category(Category),
    Universe(Universe),
    /// X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(DecCurry),
    /// in memory of Dennis M.Ritchie
    Ritchie(DecRitchie),
    /// lambda x => expr
    Abstraction(DecAbstraction),
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
    Application(DecApplication),
    ApplicationOrRitchieCall(DecApplicationOrRitchieCall),
    /// ::<ident>
    AssociatedItem(DecAssociatedItem),
    /// (<type> as <trait>)::<ident>
    TypeAsTraitItem(DecTypeAsTraitItem),
    /// <type> : <trait>
    TraitConstraint(DecTraitConstraint),
    /// `~`
    LeashOrBitNot(Toolchain),
    Wrapper(DecWrapper),
    /// can be interpreted as
    /// - a normal list of terms
    /// - List functor
    /// - Array functor
    List(DecList),
}

impl DecTerm {
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> u8 {
        match self {
            DecTerm::Curry(term) => curry_parameter_count(db, term),
            _ => 0,
        }
    }
}
