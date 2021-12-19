use atom::{BinaryOpr, Opr, PrefixOpr, SuffixOpr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    Assign = 3,
    AboveJoin = 2,
    Join = 1,
    None = 0,
}

#[test]
fn test_precedence_order() {
    assert!(Precedence::Inert > Precedence::Closed);
}

impl From<Opr> for Precedence {
    fn from(opr: Opr) -> Self {
        match opr {
            Opr::Binary(binary) => match binary {
                BinaryOpr::Call | BinaryOpr::Index => Precedence::Closed,
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
                BinaryOpr::LambdaMiddle => todo!(),
            },
            Opr::Join => Precedence::Join,
            Opr::Prefix(prefix) => match prefix {
                PrefixOpr::Shared => Precedence::Closed,
                PrefixOpr::BitNot => Precedence::Prefix,
                PrefixOpr::Minus => Precedence::Closed,
                PrefixOpr::Not => Precedence::Closed,
                PrefixOpr::Exclusive => Precedence::Closed,
                PrefixOpr::LambdaBegin => todo!(),
            },
            Opr::Suffix(suffix) => Precedence::Closed,
            Opr::Bra(_) | Opr::Ket(_) => Precedence::None,
        }
    }
}
