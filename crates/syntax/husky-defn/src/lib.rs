mod form;
mod trai;
mod ty;

pub use form::*;
pub use trai::*;
pub use ty::*;

use husky_entity_path::EntityPath;
use husky_expr::{ExprArena, ExprIdx};
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(Defn);

pub trait DefnDb: DbWithJar<DefnJar> {}

#[salsa::tracked(jar = DefnJar)]
pub struct Defn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub variant: DefnVariant,
    #[return_ref]
    pub arena: ExprArena,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefnVariant {
    Submodule,
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
}
