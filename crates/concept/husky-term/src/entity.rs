mod intern;
mod namespace;
mod root;

pub use intern::*;
pub use namespace::*;

use husky_word::{Identifier, RootIdentifier};
use optional::Optioned;

use crate::{TermQuery, Ty};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermEntityPath {
    opt_parent: Optioned<TermEntityPathPtr>,
    ident: Identifier,
}

impl TermEntityPath {
    pub fn root(ident: Identifier) -> Self {
        Self {
            opt_parent: Optioned::none(),
            ident,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermEntity {
    path: TermEntityPath,
    ty: Ty,
}

impl TermEntity {
    pub fn ty(&self) -> Ty {
        self.ty
    }
}
