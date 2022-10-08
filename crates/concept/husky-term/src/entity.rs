mod namespace;
mod root;

use husky_entity_path::EntityPathPtr;
pub use namespace::*;

use husky_word::{Identifier, RootBuiltinIdentifier};
use optional::Optioned;

use crate::{Term, TermQuery, Ty};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermEntity {
    path: EntityPathPtr,
    ty: Ty,
}

impl TermEntity {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn path(&self) -> EntityPathPtr {
        self.path
    }
}

impl Into<Term> for TermEntity {
    fn into(self) -> Term {
        Term::Entity(self)
    }
}
