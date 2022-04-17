use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTypeDecl {
    pub base_route: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub trait_impls: &'static [StaticTraitImplDecl],
    pub type_members: &'static [StaticTypeMemberDecl],
    pub variants: &'static [StaticEnumVariantDecl],
    pub kind: TypeKind,
    pub visualizer: StaticVisualizer,
    pub opt_type_call: Option<&'static StaticCallDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticFieldDecl {
    pub name: &'static str,
    pub variant: StaticFieldVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitImplDecl {
    pub route: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticFieldVariant {}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticEnumVariantDecl {}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTypeMemberDecl {
    Field,
    Method(StaticMethodDecl),
    Call,
}

impl StaticTypeMemberDecl {
    pub fn name(&self) -> &'static str {
        match self {
            StaticTypeMemberDecl::Field => todo!(),
            StaticTypeMemberDecl::Method(method) => method.name,
            StaticTypeMemberDecl::Call => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTraitMemberDecl {
    Method(StaticMethodDecl),
    Call,
    Type,
}
