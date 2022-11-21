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
    Trait(TraitDefn),
    Form(FormDefn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDefn {
    Enum(EnumDefn),
    StructLike(StructDefn),
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDefn {
    variants: Vec<VariantDefn>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDefn {
    fields: Vec<FieldDefn>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FormDefn {
    body: ExprIdx,
}
