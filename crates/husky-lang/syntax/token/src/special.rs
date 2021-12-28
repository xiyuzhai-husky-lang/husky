use common::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Special {
    LAngle,
    Leq,
    RAngle,
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
    SubOrMinus,
    Mult,
    Div,
    Power,
    And,
    DoubleVertical,
    BitNot,
    Modulo,
    MemberAccess,
    LightArrow,  // ->
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
