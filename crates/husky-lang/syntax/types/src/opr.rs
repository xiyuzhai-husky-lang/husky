use scope::ScopePtr;
use vm::{BinaryOpr, PureBinaryOpr};
use word::{CustomIdentifier, Identifier};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Opr {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    List(ListOpr),
}

impl std::fmt::Debug for Opr {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(arg0) => {
                f.write_str("Binary ")?;
                arg0.fmt(f)
            }
            // Self::Assign(opt_binary) => {
            //     f.write_str("Assign ")?;
            //     if let Some(binary) = opt_binary {
            //         binary.fmt(f)
            //     } else {
            //         Ok(())
            //     }
            // }
            Self::Prefix(arg0) => {
                f.write_str("Prefix ")?;
                arg0.fmt(f)
            }
            Self::Suffix(arg0) => {
                f.write_str("Suffix ")?;
                arg0.fmt(f)
            }
            Self::List(arg0) => {
                f.write_str("List ")?;
                arg0.fmt(f)
            }
        }
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

impl From<BinaryOpr> for Opr {
    fn from(binary: BinaryOpr) -> Self {
        Self::Binary(binary)
    }
}

impl From<PrefixOpr> for Opr {
    fn from(prefix: PrefixOpr) -> Self {
        Self::Prefix(prefix)
    }
}

impl From<SuffixOpr> for Opr {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

impl From<ListOpr> for Opr {
    fn from(list: ListOpr) -> Self {
        Self::List(list)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                            // ++
    Decr,                            // --
    MayReturn,                       // ?
    MembVarAccess(CustomIdentifier), // .
    WithType(ScopePtr),              // :
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrefixOpr {
    Minus,     // -
    Not,       // !$0
    BitNot,    // ~
    Shared,    // &
    Exclusive, // !$0 after WithType or Vec or Array
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
