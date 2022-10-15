mod application;
mod unresolved;

use application::*;
use husky_term::TermPtr;
use unresolved::*;

pub enum TermPattern {
    Unresolved(usize),
    Resolved(TermPtr),
    Application(TermApplicationPattern),
}

pub type TermPatternPtr = *const ();
