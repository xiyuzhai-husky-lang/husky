use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VariableSymbol {
    ident: Identifier,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VariableIdx(usize);

pub enum VariableData {
    Normal { ident: Identifier },
    FrameVariable { ident: Identifier },
    ThisValue { ident: Identifier },
    ThisMethod { ident: Identifier },
    ThisField { ident: Identifier },
}
