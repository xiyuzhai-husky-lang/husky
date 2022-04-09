use crate::*;

pub struct StaticTyDecl {
    pub base_ty: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub traits: &'static [&'static str],
    pub fields: &'static [StaticFieldDecl],
    pub methods: &'static [StaticMethodDecl],
    pub variants: &'static [StaticEnumVariantDecl],
    pub kind: TyKind,
}

pub struct StaticFieldDecl {
    pub name: &'static str,
    pub variant: StaticFieldVariant,
}

pub enum StaticFieldVariant {}

pub struct StaticEnumVariantDecl {}
