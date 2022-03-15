use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub ident: CustomIdentifier,
    pub ty: ScopePtr,
    pub qual: Qual,
}

impl Variable {
    pub(crate) fn from_input(input_placeholder: &InputPlaceholder) -> Self {
        Variable {
            ident: input_placeholder.ident,
            ty: input_placeholder.ranged_ty.scope,
            qual: Qual::from_input(input_placeholder.contract),
        }
    }
}
