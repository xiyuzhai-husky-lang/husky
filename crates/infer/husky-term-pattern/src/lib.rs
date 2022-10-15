mod application;
mod curry;
mod unresolved;

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
