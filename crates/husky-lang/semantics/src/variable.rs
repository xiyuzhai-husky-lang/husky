use crate::*;

pub struct Variable {
    pub ident: CustomIdentifier,
    pub ty: ScopePtr,
    pub qual: Qual,
}
