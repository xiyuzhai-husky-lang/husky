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
    LightArrow,   // ->
    HeavyArrow,   // =>
    DoubleColon,  // ::
    Colon,        // :
    Comma,        // ,
    Ambersand,    // &
    Incr,         // ++
    Decr,         // --
    Vertical,     // |
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    Exclamation,  // !
}

impl Special {
    pub fn code(&self) -> &'static str {
        match self {
            Special::LAngle => "<",
            Special::Leq => "<=",
            Special::RAngle => ">",
            Special::Geq => ">=",
            Special::Neq => "!=",
            Special::Eq => "==",
            Special::Shl => "<<",
            Special::Shr => ">>",
            Special::LCurl => "{",
            Special::RCurl => "}",
            Special::LBox => "[",
            Special::RBox => "]",
            Special::LPar => "(",
            Special::RPar => ")",
            Special::Add => "+",
            Special::SubOrMinus => "-",
            Special::Mul => "*",
            Special::Div => "/",
            Special::Power => "**",
            Special::And => "&&",
            Special::DoubleVertical => "||",
            Special::BitNot => "~",
            Special::Modulo => "%",
            Special::MemberAccess => ".",
            Special::LightArrow => "->",
            Special::HeavyArrow => "=>",
            Special::DoubleColon => "::",
            Special::Colon => ":",
            Special::Comma => ",",
            Special::Ambersand => "&",
            Special::Incr => "++",
            Special::Decr => "--",
            Special::Vertical => "|",
            Special::Assign => "=",
            Special::AddAssign => "+=",
            Special::SubAssign => "-=",
            Special::MulAssign => "*=",
            Special::DivAssign => "/=",
            Special::Exclamation => "!",
            Special::BitOrAssign => "|=",
            Special::BitAndAssign => "&=",
        }
    }
}
