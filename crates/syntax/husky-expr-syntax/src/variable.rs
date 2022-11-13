use crate::*;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawVariableKind {
    Normal { init_range: TextRange },
    ThisField,
    Unrecognized,
    Block,
}
