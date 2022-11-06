mod application;
mod context;
mod curry;
mod db;
mod subentity;
mod trait_impl;
mod unresolved;

pub use context::*;
pub use db::*;

use application::*;
use curry::*;
use husky_term::*;
use subentity::*;
use trait_impl::*;
use unresolved::*;

pub enum TermPattern<'a> {
    Resolved(TermItd),
    Unresolved(UnresolvedTerm),
    Application(TermApplicationPattern<'a>),
    Curry(TermCurryPattern<'a>),
    Subentity(TermSubentityPattern<'a>), // ::
    TraitImpl(TermTraitImplPattern<'a>), // A as trait
}
