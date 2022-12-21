use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                 // everyone can access it
    PubicUnder(ModulePath), // everyone under a path can access it
    Protected,              // this is default: only self and its submodules
    Private,                // only self
}
