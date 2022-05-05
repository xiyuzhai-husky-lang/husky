use super::*;

impl From<ListOpr> for Opr {
    fn from(list: ListOpr) -> Self {
        Self::List(list)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ListOpr {
    TupleInit,
    NewVec,
    NewDict,
    Call,
    Index,
    ModuloIndex,
    StructInit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListStartAttr {
    None,
    Attach,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Curl,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Curl => "{",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Curl => "}",
        }
    }
}
