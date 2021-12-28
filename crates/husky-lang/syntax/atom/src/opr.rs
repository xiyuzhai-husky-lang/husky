use scope::ScopeId;
use word::Identifier;

// LightArrow, // ->
// HeavyArrow, // =>

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                     // ++
    Decr,                     // --
    MayReturn,                // ?
    MemberAccess(Identifier), // .
    WithType(ScopeId),        // :
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
pub enum JoinOpr {
    Comma, // ,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOpr {
    Less,       // <
    Leq,        // <=
    Greater,    // >
    Geq,        // >=
    Neq,        // !=
    Eq,         // ==
    LShift,     // >>
    RShift,     // <<
    Add,        // +
    Sub,        // -
    Mult,       // *
    Div,        // /
    Power,      // **
    And,        // && and
    BitAnd,     // &
    Or,         // ||
    BitXor,     // ^
    BitOr,      // |
    Modulo,     // %
    Assign,     // =
    AddAssign,  // +=
    SubAssign,  // -=
    MultAssign, // *=
    DivAssign,  // /=
}
impl BinaryOpr {
    pub fn spaced_code(&self) -> &'static str {
        match self {
            BinaryOpr::Less => " < ",
            BinaryOpr::Leq => " <= ",
            BinaryOpr::Greater => " > ",
            BinaryOpr::Geq => " >= ",
            BinaryOpr::Neq => " != ",
            BinaryOpr::Eq => " == ",
            BinaryOpr::LShift => " << ",
            BinaryOpr::RShift => " >>",
            BinaryOpr::Add => " + ",
            BinaryOpr::Sub => " - ",
            BinaryOpr::Mult => " * ",
            BinaryOpr::Div => " / ",
            BinaryOpr::Power => " ** ",
            BinaryOpr::And => " && ",
            BinaryOpr::BitAnd => " & ",
            BinaryOpr::Or => " || ",
            BinaryOpr::BitXor => " ^ ",
            BinaryOpr::BitOr => " | ",
            BinaryOpr::Modulo => " % ",
            BinaryOpr::Assign => " = ",
            BinaryOpr::AddAssign => " += ",
            BinaryOpr::SubAssign => " -= ",
            BinaryOpr::MultAssign => " *= ",
            BinaryOpr::DivAssign => " /= ",
        }
    }
}
