mod form;
mod trai;
mod ty;

pub use form::*;
pub use trai::*;
pub use ty::*;

use husky_entity_path::EntityPathItd;
use husky_expr_syntax::{ExprArena, ExprIdx};
use timed_salsa::DbWithJar;

#[timed_salsa::jar(db = DefnDb)]
pub struct Jar(Defn);

pub trait DefnDb: DbWithJar<Jar> {}

#[timed_salsa::tracked]
pub struct Defn {
    #[id]
    pub entity_path: EntityPathItd,
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
