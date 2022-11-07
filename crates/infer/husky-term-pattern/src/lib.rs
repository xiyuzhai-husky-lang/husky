mod application;
mod context;
mod curry;
mod db;
mod intern;
mod subentity;
mod trait_impl;
mod unresolved;

pub use context::*;
pub use db::*;
pub use intern::*;

use application::*;
use curry::*;
use husky_term::*;
use subentity::*;
use trait_impl::*;
use unresolved::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TermPattern {
    Resolved(TermItd),
    Unresolved(UnresolvedTermIdx),
    Application(TermApplicationPattern),
    Curry(TermCurryPattern),
    Subentity(TermSubentityPattern), // ::
    TraitImpl(TermTraitImplPattern), // A as trait
}
