use common::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Special {
    LAngle,
    Leq,
    RAngle,
    Geq,
    Neq,
    Eq,
    Shl,
    Shr,
    LCurl,
    RCurl,
    LBox,
    RBox,
    LPar,
    RPar,
    Add,
    SubOrMinus,
    Mul,
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
    AddAssign,   // +=
    SubAssign,   // -=
    MulAssign,   // *=
    DivAssign,   // /=
    Exclamation, // !
}

impl Special {
    pub fn code(&self) -> &'static str {
        match self {
            Special::LAngle => todo!(),
            Special::Leq => todo!(),
            Special::RAngle => todo!(),
            Special::Geq => todo!(),
            Special::Neq => todo!(),
            Special::Eq => todo!(),
            Special::Shl => todo!(),
            Special::Shr => todo!(),
            Special::LCurl => todo!(),
            Special::RCurl => todo!(),
            Special::LBox => todo!(),
            Special::RBox => todo!(),
            Special::LPar => todo!(),
            Special::RPar => todo!(),
            Special::Add => todo!(),
            Special::SubOrMinus => todo!(),
            Special::Mul => todo!(),
            Special::Div => todo!(),
            Special::Power => todo!(),
            Special::And => todo!(),
            Special::DoubleVertical => todo!(),
            Special::BitNot => todo!(),
            Special::Modulo => todo!(),
            Special::MemberAccess => todo!(),
            Special::LightArrow => todo!(),
            Special::HeavyArrow => todo!(),
            Special::DoubleColon => todo!(),
            Special::Colon => todo!(),
            Special::Comma => todo!(),
            Special::Ambersand => todo!(),
            Special::Incr => todo!(),
            Special::Decr => todo!(),
            Special::Vertical => todo!(),
            Special::Assign => "=",
            Special::AddAssign => "+=",
            Special::SubAssign => "-=",
            Special::MulAssign => "*=",
            Special::DivAssign => "/=",
            Special::Exclamation => "!",
        }
    }
}
