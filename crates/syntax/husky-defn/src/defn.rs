mod form;
mod trai;
mod ty;

pub use form::*;
pub use trai::*;
pub use ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
}

impl Defn {
    pub fn entity_path(self, db: &dyn DefnDb) -> EntityPath {
        match self {
            Defn::Type(defn) => todo!(),
            Defn::Trait(_) => todo!(),
            Defn::Form(_) => todo!(),
        }
    }
}
