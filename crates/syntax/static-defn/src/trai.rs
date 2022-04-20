use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticTraitDecl {
    pub base_route: &'static str,
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub members: &'static [StaticTraitMemberDecl],
}
