mod abstraction;
mod application;
mod as_trai_subentity;
mod curry;
mod jordan;
mod subentity;
mod trai_constraint;

pub use abstraction::*;
pub use application::*;
pub use as_trai_subentity::*;
pub use curry::*;
pub use jordan::*;
pub use subentity::*;
pub use trai_constraint::*;

use crate::*;

pub enum LocalTerm {
    Term(Term),
    ImplicitLifetime(u8),
    Curry(LocalTermCurryIdx),
    Application(LocalTermApplicationIdx),
    Abstraction(LocalTermAbstractionIdx),
    Jordan(LocalTermJordanIdx),
    Subentity(LocalTermSubentityIdx),
    AsTraitSubentity(LocalTermAsTraitSubentityIdx),
    TraitConstraint(LocalTermTraitConstraint),
}
