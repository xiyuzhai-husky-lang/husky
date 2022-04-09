use crate::*;

pub struct StaticTyDecl {
    pub route: &'static str,
    pub traits: &'static [&'static str],
    pub fields: &'static [StaticFieldDecl],
    pub methods: &'static [StaticMethodDecl],
    pub variants: &'static [StaticEnumVariantDecl],
    pub kind: TyKind,
}

pub struct StaticFieldDecl {}

pub struct StaticEnumVariantDecl {}
