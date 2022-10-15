mod application;
mod context;
mod curry;
mod db;
mod unresolved;

pub use context::*;
pub use db::*;

use application::*;
use curry::*;
use husky_term::*;
use unresolved::*;

pub enum TermPattern<'a> {
    Unresolved(UnresolvedTerm),
    Resolved(TermPtr),
    Application(TermApplicationPattern<'a>),
    Curry(TermCurryPattern<'a>),
}
