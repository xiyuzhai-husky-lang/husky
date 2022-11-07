mod application;
mod context;
mod curry;
mod db;
mod storage;
mod subentity;
mod trait_impl;
mod unresolved;

pub use context::*;
pub use db::*;
pub use storage::*;

use application::*;
use curry::*;
use husky_term::*;
use subentity::*;
use trait_impl::*;
use unresolved::*;

pub enum TermPattern {
    Resolved(TermItd),
    Unresolved(UnresolvedTerm),
    Application(TermApplicationPattern),
    Curry(TermCurryPattern),
    Subentity(TermSubentityPattern), // ::
    TraitImpl(TermTraitImplPattern), // A as trait
}
