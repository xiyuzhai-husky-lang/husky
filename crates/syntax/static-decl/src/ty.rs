use crate::*;

pub struct StaticTyDecl {
    pub base_ty: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub trait_impls: &'static [StaticTraitImplDecl],
    pub fields: &'static [StaticFieldDecl],
    pub type_members: &'static [StaticTypeMemberDecl],
    pub variants: &'static [StaticEnumVariantDecl],
    pub kind: TyKind,
}

pub struct StaticFieldDecl {
    pub name: &'static str,
    pub variant: StaticFieldVariant,
}

pub struct StaticTraitImplDecl {
    pub route: &'static str,
}

pub enum StaticFieldVariant {}

pub struct StaticEnumVariantDecl {}

pub enum StaticTypeMemberDecl {
    Field,
    Method(StaticMethodDecl),
    Call,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTraitMemberDecl {
    Method(StaticMethodDecl),
    Call,
    Type,
}
