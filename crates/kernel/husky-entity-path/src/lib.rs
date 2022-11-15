mod db;
mod display;
mod intern;
mod menu;

pub use db::*;
pub use menu::*;

use husky_word::Identifier;
pub use intern::*;
use optional::Optioned;

// EntityPath examples: std::ops::Add

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

    #[inline(always)]
    pub fn opt_parent(&self) -> Option<EntityPathItd> {
        self.opt_parent.into_option()
    }
}
