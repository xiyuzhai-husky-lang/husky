mod display;
mod intern;

use husky_word::{Identifier, RootBuiltinIdentifier};
pub use intern::*;
use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityPath {
    opt_parent: Optioned<EntityPathPtr>,
    ident: Identifier,
}

impl EntityPath {
    fn root(ident: Identifier) -> Self {
        Self {
            opt_parent: Optioned::none(),
            ident,
        }
    }
}
