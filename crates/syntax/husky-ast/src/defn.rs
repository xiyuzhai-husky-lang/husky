use crate::*;

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
    Form(FormDefn),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct FormDefn {
    body: ExprIdx,
}
