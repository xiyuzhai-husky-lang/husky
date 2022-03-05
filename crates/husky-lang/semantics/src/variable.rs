use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub ident: CustomIdentifier,
    pub ty: ScopePtr,
    pub qual: Qual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarIdx(u8);

impl VarIdx {
    pub(crate) fn new(raw: usize) -> VarIdx {
        VarIdx(u8::try_from(raw).unwrap())
    }
}

impl From<usize> for VarIdx {
    fn from(raw: usize) -> Self {
        Self::new(raw)
    }
}
