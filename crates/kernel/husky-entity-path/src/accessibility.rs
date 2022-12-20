use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                // everyone can access it
    PubicWith(EntityPath), // everyone under a path can access it
    Protected,             // this is default: only self and its submodules
    Private,               // only self
}
