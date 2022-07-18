pub use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VariadicTemplateDecl {
    None,
}

impl VariadicTemplateDecl {
    pub(crate) fn from_static(static_defn: &StaticVariadicTemplateDefn) -> Self {
        match static_defn {
            StaticVariadicTemplateDefn::None => VariadicTemplateDecl::None,
        }
    }
}
