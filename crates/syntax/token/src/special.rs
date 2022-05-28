use vm::Bracket;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Special {
    LAngle,            // <
    Leq,               // <=
    RAngle,            // >
    Geq,               // >=
    Neq,               // !=
    MaybeEq,           // ?=
    Eq,                // ==
    Shl,               // <<
    Shr,               //>>
    LCurl,             // {
    RCurl,             // }
    LBox,              // [
    RBox,              //]
    LPar,              // (
    RPar,              //)
    Add,               // +
    SubOrMinus,        // -
    Star,              // *
    Div,               // /
    Power,             // **
    And,               // &&
    DoubleVertical,    // ||
    BitNot,            // ~
    Modulo,            // %
    MemberAccess,      // .
    LightArrow,        // ->
    HeavyArrow,        // =>
    DoubleColon,       // ::
    Colon,             // :
    Comma,             // ,
    Ambersand,         // &
    Incr,              // ++
    Decr,              // --
    Vertical,          // |
    Assign,            // =
    AddAssign,         // +=
    SubAssign,         // -=
    MulAssign,         // *=
    DivAssign,         // /=
    BitAndAssign,      // &=
    BitOrAssign,       // |=
    Exclamation,       // !
    DoubleExclamation, // !!
    Semicolon,         // ;
    XmlKet,            // />
}

impl Special {
    pub fn code(&self) -> &'static str {
        match self {
            Special::LAngle => "<",
            Special::Leq => "<=",
            Special::RAngle => ">",
            Special::Geq => ">=",
            Special::Neq => "!=",
            Special::MaybeEq => "?=",
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
            Special::Star => "*",
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
            Special::DoubleExclamation => "!!",
            Special::Semicolon => ";",
            Special::XmlKet => "/>",
        }
    }

    pub fn opt_bra(&self) -> Option<Bracket> {
        match self {
            Special::LAngle => Some(Bracket::Angle),
            Special::LCurl => Some(Bracket::Curl),
            Special::LBox => Some(Bracket::Box),
            Special::LPar => Some(Bracket::Par),
            _ => None,
        }
    }
}
