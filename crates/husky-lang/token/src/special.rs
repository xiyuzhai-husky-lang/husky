use common::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Special {
    LessOrLAngle,
    Leq,
    GreaterOrRAngle,
    Geq,
    Neq,
    Eq,
    LShift,
    RShift,
    LCurl,
    RCurl,
    LBox,
    RBox,
    LPar,
    RPar,
    Add,
    Sub,
    Mult,
    Div,
    Power,
    And,
    DoubleVertical,
    BitNot,
    Modulo,
    MemberAccess,
    HeavyArrow,  // =>
    DoubleColon, // ::
    Colon,       // :
    Comma,       // ,
    Ambersand,   // &
    Incr,        // ++
    Decr,        // --
    Vertical,    // |
    Assign,      // =
    AddAssign,
    SubAssign,
    MultAssign,
    DivAssign,
    Exclamation,
}
