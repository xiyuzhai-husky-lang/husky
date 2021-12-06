use common::*;

pub struct Token {
    pub range: TextRange,
    pub precedence: Precedence,
    pub cls: TokenVariant,
    pub is_rear_attached: bool,
}
impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Token {{range: {:?}, cls: {:?}, prcd: {:?}, ira: {:?}}}",
            &self.range, &self.cls, &self.precedence, &self.is_rear_attached
        ))
        // f.debug_struct("Token")
        //   .field("range", &self.range)
        //   // .field("precedence", &self.precedence)
        //   .field("cls", &self.cls)
        //   .field("is_rear_attached", &self.is_rear_attached)
        //   .finish()
    }
}
// pub fn get_precedence(cls: &TokenVariant) -> Precedence {
//     match cls {
//         TokenVariant::Join(join) => match join {
//             Join::List => Precedence::List,
//             Join::Comma => Precedence::Comma,
//             Join::ModuleQualifier => Precedence::Closed,
//         },
//         TokenVariant::Binary(binary) => match binary {
//             Binary::Call | Binary::Index | Binary::Combine | Binary::MemberAccess => {
//                 Precedence::Closed
//             }
//             Binary::Be => Precedence::Be,
//             Binary::Arrow => Precedence::Arrow,
//             Binary::Eq | Binary::Neq => Precedence::Equal,
//             Binary::And => Precedence::And,
//             Binary::BitAnd => Precedence::BitAnd,
//             Binary::Or => Precedence::Or,
//             Binary::BitOr => Precedence::BitOr,
//             Binary::Mult | Binary::Div | Binary::Modulo => Precedence::Multiplicative,
//             Binary::SubAssign => Precedence::Be,
//             Binary::Add | Binary::Sub => Precedence::Additive,
//             Binary::LShift | Binary::RShift => Precedence::Shift,
//             Binary::Leq | Binary::Less | Binary::Geq | Binary::Greater => Precedence::Compare,
//             _ => {
//                 td!();
//             }
//         },
//         TokenVariant::Keyword(_) => Precedence::Inert,
//         TokenVariant::Prefix(prefix) => match prefix {
//             Prefix::Shared => Precedence::Closed,
//             Prefix::BitNot => Precedence::Prefix,
//             Prefix::Minus => Precedence::Closed,
//             Prefix::NotOrExclusive => Precedence::Closed,
//         },
//         TokenVariant::Suffix(suffix) => match suffix {
//             Suffix::Incr => Precedence::Closed,
//             Suffix::Decr => Precedence::Closed,
//         },
//         TokenVariant::Atom(_atom) => Precedence::Inert,
//         TokenVariant::Bra(bra) => match bra {
//             _ => Precedence::None,
//         },
//         TokenVariant::Ket(ket) => match ket {
//             _ => Precedence::None,
//         },
//         TokenVariant::PhraseStart => Precedence::None,
//     }
// }

// impl Token {
//     fn new(range: file::Range, cls: TokenVariant, is_rear_attached: bool) -> Token {
//         Token {
//             range,
//             precedence: get_precedence(&cls),
//             cls,
//             is_rear_attached,
//         }
//     }
//     pub fn implicit_token_before(before_which: &Token, cls: TokenVariant) -> Token {
//         Token::new(
//             file::Range {
//                 start: before_which.range.start,
//                 end: before_which.range.start,
//             },
//             cls,
//             false,
//         )
//     }
//     pub fn implicit_token_after(after_which: &Token, cls: TokenVariant) -> Token {
//         Token::new(
//             file::Range {
//                 start: after_which.range.end,
//                 end: after_which.range.end,
//             },
//             cls,
//             false,
//         )
//     }
// }

#[derive(Debug)]
pub enum TokenVariant {
    PhraseStart,
    Keyword(Keyword),
    Atom(Atom),
    Prefix(Prefix),
    Suffix(Suffix),
    Binary(Binary),
    Join(Join),
    Bra(Bracket),
    Ket(Bracket),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Curl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Join {
    ModuleQualifier, // :: no space between before (Ket/Name) && after (Name)
    List,            // space between Atom/Ket && Atom/Bra, List as in Lisp
    Comma,           // ,
}

impl Join {
    pub fn code(&self) -> String {
        match self {
            Join::ModuleQualifier => "::".to_string(),
            Join::List => " ".to_string(),
            Join::Comma => ", ".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Binary {
    MemberAccess, // .
    Call,         // implied by no space between Atom/Ket && LPar
    Index,        // implied by no space between Atom/Ket && LBox
    Combine,      // implied by no space between Atom/Ket && Atom/Prefix
    Mult,         // *
    Div,          // /
    Add,          // +
    Sub,          // -
    Modulo,       // %
    LShift,       // <<
    RShift,       // >>
    Less,         // <
    Leq,          // <=
    Greater,      // >
    Geq,          // >=
    Eq,           // ==
    Neq,          // !=
    BitAnd,       // & not rear attached
    BitXor,       // ^
    BitOr,        // |
    And,          // && &&
    Or,           // or ||
    Arrow,        // =>
    Be,           // =
    AddAssign,    // +=
    SubAssign,    // -=
    MultAssign,   // *=
    DivAssign,    // /=
    ModuloAssign, // %=
    LShiftAssign, // <<=
    RShiftAssign, // >>=
    BitAndAssign, // &=
    BitXorAssign, // ^=
    BitOrAssign,  // |=
}
impl Binary {
    pub fn code(&self) -> String {
        match self {
            Binary::MemberAccess => "::".to_string(),
            Binary::Call => "".to_string(),
            Binary::Index => td!(),
            Binary::Combine => td!(),
            Binary::Mult => td!(),
            Binary::Div => td!(),
            Binary::Add => td!(),
            Binary::Sub => td!(),
            Binary::Modulo => td!(),
            Binary::LShift => td!(),
            Binary::RShift => td!(),
            Binary::Less => td!(),
            Binary::Leq => td!(),
            Binary::Greater => td!(),
            Binary::Geq => td!(),
            Binary::Eq => td!(),
            Binary::Neq => td!(),
            Binary::BitAnd => td!(),
            Binary::BitXor => td!(),
            Binary::BitOr => td!(),
            Binary::And => td!(),
            Binary::Or => td!(),
            Binary::Arrow => "=>".to_string(),
            Binary::Be => td!(),
            Binary::AddAssign => td!(),
            Binary::SubAssign => td!(),
            Binary::MultAssign => td!(),
            Binary::DivAssign => td!(),
            Binary::ModuloAssign => td!(),
            Binary::LShiftAssign => td!(),
            Binary::RShiftAssign => td!(),
            Binary::BitAndAssign => td!(),
            Binary::BitXorAssign => td!(),
            Binary::BitOrAssign => td!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Prefix {
    Minus,          // - is first or no space after yet space before
    NotOrExclusive, // !
    BitNot,         // ~
    Shared,         // & rear attached
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suffix {
    Incr, // ++
    Decr, // -- s
}

#[derive(Debug)]
pub enum Atom {
    Void,
    Identifier(SymbolID),
    Int(i32),
    Float(f32),
    Bool(bool),
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
    Be = 2,
    Monad = 1,
    None = 0,
}

#[derive(PartialEq)]
pub enum Convexity {
    None,
    Convex,
    Concave,
}
