use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListOpr {
    NewTuple,
    NewVec,
    NewLambdaHead,
    NewDict,
    FunctionCall,
    Index,
    ModuloIndex,
    StructInit,
    MethodCall { ranged_ident: RangedIdentifier },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListStartAttr {
    None,
    Attach,
    MethodAttach { ranged_ident: RangedIdentifier },
}

impl ListStartAttr {
    pub fn attached(&self) -> bool {
        match self {
            ListStartAttr::None => false,
            ListStartAttr::Attach => true,
            ListStartAttr::MethodAttach { .. } => true,
        }
    }
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
    Angle,
    Curl,
    Vertical,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Angle => "<",
            Bracket::Curl => "{",
            Bracket::Vertical => "|",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Angle => ">",
            Bracket::Curl => "}",
            Bracket::Vertical => "|",
        }
    }
}
