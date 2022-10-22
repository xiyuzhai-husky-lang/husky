mod db;
mod display;
mod intern;
mod menu;

pub use db::*;
pub use menu::*;

use husky_word::{Identifier, RootBuiltinIdentifier};
pub use intern::*;
use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityPath {
    opt_parent: Optioned<EntityPathItd>,
    ident: Identifier,
}

impl EntityPath {
    pub fn root(ident: Identifier) -> Self {
        Self {
            opt_parent: Optioned::none(),
            ident,
        }
    }
}
