mod form;
mod trai;
mod trai_item;
mod ty;
mod ty_item;

pub use form::*;
pub use trai::*;
pub use trai_item::*;
pub use ty::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
    TypeItem(TypeItemDefn),
    TraitItem(TraitItemDefn),
}

impl Defn {
    pub fn entity_path(self, db: &dyn DefnDb) -> EntityPath {
        match self {
            Defn::Type(defn) => todo!(),
            Defn::Trait(_) => todo!(),
            Defn::Form(_) => todo!(),
            Defn::TypeItem(_) => todo!(),
            Defn::TraitItem(_) => todo!(),
        }
    }
}
