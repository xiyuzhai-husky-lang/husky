pub enum Opr {
    Join(JoinOpr),
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    Bra(Bracket),
    Ket(Bracket),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,      // ++
    Decr,      // --
    MayReturn, // ?
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrefixOpr {
    Minus,       // -
    Not,         // !$0
    BitNot,      // ~
    Shared,      // &
    Exclusive,   // !$0 after OfType or Vec or Array
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
    MemberAccess, // .
    LightArrow,   // ->
    HeavyArrow,   // =>
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MultAssign,   // *=
    DivAssign,    // /=
    OfType,       // :
    LambdaMiddle, // |$0| $1
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Inert = 20,
    Closed = 19,
    Prefix = 18,
    Power = 17,
    Multiplicative = 16,
    Additive = 15,
    Shift = 14,
    Compare = 13,
    Equal = 12,
    BitAnd = 11,
    BitXor = 10,
    BitOr = 9,
    And = 8,
    Or = 7,
    TernaryConditional = 6,
    List = 5,
    Arrow = 4,
    Comma = 3,
    Assign = 2,
    Lambda = 1,
    None = 0,
}

#[test]
fn test_precedence_order() {
    assert!(Precedence::Inert > Precedence::Closed);
}

impl From<Opr> for Precedence {
    fn from(opr: Opr) -> Self {
        match opr {
            Opr::Join(join) => match join {
                JoinOpr::Comma => Precedence::Comma,
            },
            Opr::Binary(binary) => match binary {
                BinaryOpr::Call | BinaryOpr::Index | BinaryOpr::MemberAccess => Precedence::Closed,
                BinaryOpr::Assign => Precedence::Assign,
                BinaryOpr::LightArrow => Precedence::Arrow,
                BinaryOpr::HeavyArrow => Precedence::Arrow,
                BinaryOpr::Eq | BinaryOpr::Neq => Precedence::Equal,
                BinaryOpr::And => Precedence::And,
                BinaryOpr::BitAnd => Precedence::BitAnd,
                BinaryOpr::Or => Precedence::Or,
                BinaryOpr::BitOr => Precedence::BitOr,
                BinaryOpr::Mult | BinaryOpr::Div | BinaryOpr::Modulo => Precedence::Multiplicative,
                BinaryOpr::SubAssign
                | BinaryOpr::AddAssign
                | BinaryOpr::MultAssign
                | BinaryOpr::DivAssign => Precedence::Assign,
                BinaryOpr::Add | BinaryOpr::Sub => Precedence::Additive,
                BinaryOpr::LShift | BinaryOpr::RShift => Precedence::Shift,
                BinaryOpr::Leq | BinaryOpr::Less | BinaryOpr::Geq | BinaryOpr::Greater => {
                    Precedence::Compare
                }
                BinaryOpr::Power => todo!(),
                BinaryOpr::OfType => todo!(),
                BinaryOpr::LambdaMiddle => todo!(),
            },
            Opr::Prefix(prefix) => match prefix {
                PrefixOpr::Shared => Precedence::Closed,
                PrefixOpr::BitNot => Precedence::Prefix,
                PrefixOpr::Minus => Precedence::Closed,
                PrefixOpr::Not => Precedence::Closed,
                PrefixOpr::Exclusive => Precedence::Closed,
                PrefixOpr::LambdaBegin => todo!(),
            },
            Opr::Suffix(suffix) => match suffix {
                SuffixOpr::Incr => Precedence::Closed,
                SuffixOpr::Decr => Precedence::Closed,
                SuffixOpr::MayReturn => Precedence::Closed,
            },
            Opr::Bra(_) | Opr::Ket(_) => Precedence::None,
        }
    }
}
