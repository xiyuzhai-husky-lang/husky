use scope::ScopeId;
use token::Special;
use word::Identifier;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opr {
    Binary(BinaryOpr),
    Join,
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    Bra(Bracket),
    Ket(Bracket),
}

impl From<BinaryOpr> for Opr {
    fn from(opr: BinaryOpr) -> Self {
        Self::Binary(opr)
    }
}

impl From<PrefixOpr> for Opr {
    fn from(opr: PrefixOpr) -> Self {
        Self::Prefix(opr)
    }
}

impl From<SuffixOpr> for Opr {
    fn from(opr: SuffixOpr) -> Self {
        Self::Suffix(opr)
    }
}

impl From<&Special> for Opr {
    fn from(special: &Special) -> Self {
        match special {
            token::Special::DoubleColon => panic!(),
            token::Special::Colon => panic!(),
            token::Special::Vertical => panic!(),
            token::Special::Ambersand => panic!(),
            token::Special::Exclamation => panic!(),
            token::Special::DoubleVertical => panic!(),
            token::Special::LPar => panic!(),
            token::Special::LBox => panic!(),
            token::Special::LCurl => panic!(),
            token::Special::MemberAccess => panic!(),
            token::Special::LessOrLAngle => Opr::Binary(BinaryOpr::Less),
            token::Special::Leq => Opr::Binary(BinaryOpr::Leq),
            token::Special::GreaterOrRAngle => Opr::Binary(BinaryOpr::Greater),
            token::Special::Geq => Opr::Binary(BinaryOpr::Geq),
            token::Special::Neq => Opr::Binary(BinaryOpr::Neq),
            token::Special::Eq => Opr::Binary(BinaryOpr::Eq),
            token::Special::LShift => Opr::Binary(BinaryOpr::LShift),
            token::Special::RShift => Opr::Binary(BinaryOpr::RShift),
            token::Special::RCurl => Opr::Ket(Bracket::Curl),
            token::Special::RBox => Opr::Ket(Bracket::Box),
            token::Special::RPar => Opr::Ket(Bracket::Par),
            token::Special::Add => Opr::Binary(BinaryOpr::Add),
            token::Special::Sub => Opr::Binary(BinaryOpr::Sub),
            token::Special::Mult => Opr::Binary(BinaryOpr::Mult),
            token::Special::Div => Opr::Binary(BinaryOpr::Div),
            token::Special::Power => Opr::Binary(BinaryOpr::Power),
            token::Special::And => Opr::Binary(BinaryOpr::And),
            token::Special::BitNot => Opr::Prefix(PrefixOpr::BitNot),
            token::Special::Modulo => Opr::Binary(BinaryOpr::Modulo),
            token::Special::HeavyArrow => Opr::Binary(BinaryOpr::HeavyArrow),
            token::Special::Incr => Opr::Suffix(SuffixOpr::Incr),
            token::Special::Decr => Opr::Suffix(SuffixOpr::Decr),
            token::Special::Assign => Opr::Binary(BinaryOpr::Assign),
            token::Special::AddAssign => Opr::Binary(BinaryOpr::AddAssign),
            token::Special::SubAssign => Opr::Binary(BinaryOpr::SubAssign),
            token::Special::MultAssign => Opr::Binary(BinaryOpr::MultAssign),
            token::Special::DivAssign => Opr::Binary(BinaryOpr::DivAssign),
            token::Special::Comma => Opr::Join,
        }
    }
}

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
    Minus,       // -
    Not,         // !$0
    BitNot,      // ~
    Shared,      // &
    Exclusive,   // !$0 after WithType or Vec or Array
    LambdaBegin, // |$0| $1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Curl,
    Angle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JoinOpr {
    Comma, // ,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOpr {
    Call,         // $0($1,*)
    Index,        // $0[$1,*]
    Less,         // <
    Leq,          // <=
    Greater,      // >
    Geq,          // >=
    Neq,          // !=
    Eq,           // ==
    LShift,       // >>
    RShift,       // <<
    Add,          // +
    Sub,          // -
    Mult,         // *
    Div,          // /
    Power,        // **
    And,          // && and
    BitAnd,       // &
    Or,           // ||
    BitOr,        // |
    Modulo,       // %
    LightArrow,   // ->
    HeavyArrow,   // =>
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MultAssign,   // *=
    DivAssign,    // /=
    LambdaMiddle, // |$0| $1
}
