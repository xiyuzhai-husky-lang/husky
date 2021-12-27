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
    Vert,
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
