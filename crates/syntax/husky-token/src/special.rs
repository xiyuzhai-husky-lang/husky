use husky_opn_syntax::Bracket;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum SpecialToken {
    LAngle,            // <
    Leq,               // <=
    RAngle,            // >
    Geq,               // >=
    Neq,               // !=
    DeriveAssign,      // ?=
    Eq,                // ==
    Shl,               // <<
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
    FieldAccess,       // .
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
    At,                // @
    QuestionMark,      // ?
}

impl std::fmt::Display for SpecialToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl SpecialToken {
    pub fn code(&self) -> &'static str {
        match self {
            SpecialToken::LAngle => "<",
            SpecialToken::Leq => "<=",
            SpecialToken::RAngle => ">",
            SpecialToken::Geq => ">=",
            SpecialToken::Neq => "!=",
            SpecialToken::DeriveAssign => "?=",
            SpecialToken::Eq => "==",
            SpecialToken::Shl => "<<",
            SpecialToken::LCurl => "{",
            SpecialToken::RCurl => "}",
            SpecialToken::LBox => "[",
            SpecialToken::RBox => "]",
            SpecialToken::LPar => "(",
            SpecialToken::RPar => ")",
            SpecialToken::Add => "+",
            SpecialToken::SubOrMinus => "-",
            SpecialToken::Star => "*",
            SpecialToken::Div => "/",
            SpecialToken::Power => "**",
            SpecialToken::And => "&&",
            SpecialToken::DoubleVertical => "||",
            SpecialToken::BitNot => "~",
            SpecialToken::Modulo => "%",
            SpecialToken::FieldAccess => ".",
            SpecialToken::LightArrow => "->",
            SpecialToken::HeavyArrow => "=>",
            SpecialToken::DoubleColon => "::",
            SpecialToken::Colon => ":",
            SpecialToken::Comma => ",",
            SpecialToken::Ambersand => "&",
            SpecialToken::Incr => "++",
            SpecialToken::Decr => "--",
            SpecialToken::Vertical => "|",
            SpecialToken::Assign => "=",
            SpecialToken::AddAssign => "+=",
            SpecialToken::SubAssign => "-=",
            SpecialToken::MulAssign => "*=",
            SpecialToken::DivAssign => "/=",
            SpecialToken::Exclamation => "!",
            SpecialToken::BitOrAssign => "|=",
            SpecialToken::BitAndAssign => "&=",
            SpecialToken::DoubleExclamation => "!!",
            SpecialToken::Semicolon => ";",
            SpecialToken::XmlKet => "/>",
            SpecialToken::At => "@",
            SpecialToken::QuestionMark => "?",
        }
    }

    pub fn opt_bra(&self) -> Option<Bracket> {
        match self {
            SpecialToken::LAngle => Some(Bracket::Angle),
            SpecialToken::LCurl => Some(Bracket::Curl),
            SpecialToken::LBox => Some(Bracket::Box),
            SpecialToken::LPar => Some(Bracket::Par),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! special_token {
    ("<") => {{
        SpecialToken::LAngle
    }};
    ("<=") => {{
        SpecialToken::Leq
    }};
    (">") => {{
        SpecialToken::RAngle
    }};
    (">=") => {{
        SpecialToken::Geq
    }};
    ("!=") => {{
        SpecialToken::Neq
    }};
    ("?=") => {{
        SpecialToken::DeriveAssign
    }};
    ("==") => {{
        SpecialToken::Eq
    }};
    ("<<") => {{
        SpecialToken::Shl
    }};
    (">>") => {{
        SpecialToken::Shr
    }};
    ("{") => {{
        SpecialToken::LCurl
    }};
    ("}") => {{
        SpecialToken::RCurl
    }};
    ("[") => {{
        SpecialToken::LBox
    }};
    ("]") => {{
        SpecialToken::RBox
    }};
    ("(") => {{
        SpecialToken::LPar
    }};
    (")") => {{
        SpecialToken::RPar
    }};
    ("+") => {{
        SpecialToken::Add
    }};
    ("-") => {{
        SpecialToken::SubOrMinus
    }};
    ("*") => {{
        SpecialToken::Star
    }};
    ("/") => {{
        SpecialToken::Div
    }};
    ("**") => {{
        SpecialToken::Power
    }};
    ("&&") => {{
        SpecialToken::And
    }};
    ("||") => {{
        SpecialToken::DoubleVertical
    }};
    ("~") => {{
        SpecialToken::BitNot
    }};
    ("%") => {{
        SpecialToken::Modulo
    }};
    (".") => {{
        SpecialToken::MemberAccess
    }};
    ("->") => {{
        SpecialToken::LightArrow
    }};
    (")") => {{
        SpecialToken::HeavyArrow
    }};
    ("::") => {{
        SpecialToken::DoubleColon
    }};
    (":") => {{
        SpecialToken::Colon
    }};
    (",") => {{
        SpecialToken::Comma
    }};
    ("&") => {{
        SpecialToken::Ambersand
    }};
    ("++") => {{
        SpecialToken::Incr
    }};
    ("--") => {{
        SpecialToken::Decr
    }};
    ("|") => {{
        SpecialToken::Vertical
    }};
    ("=") => {{
        SpecialToken::Assign
    }};
    ("+=") => {{
        SpecialToken::AddAssign
    }};
    ("-=") => {{
        SpecialToken::SubAssign
    }};
    ("*=") => {{
        SpecialToken::MulAssign
    }};
    ("/=") => {{
        SpecialToken::DivAssign
    }};
    ("!") => {{
        SpecialToken::Exclamation
    }};
    ("|=") => {{
        SpecialToken::BitOrAssign
    }};
    ("&=") => {{
        SpecialToken::BitAndAssign
    }};
    ("!!") => {{
        SpecialToken::DoubleExclamation
    }};
    (";") => {{
        SpecialToken::Semicolon
    }};
    ("/>") => {{
        SpecialToken::XmlKet
    }};
    ("@") => {{
        SpecialToken::At
    }};
    ("?") => {{
        SpecialToken::QuestionMark
    }};
}

#[macro_export]
macro_rules! is_special {
    ($token: expr, $s: tt) => {{
        $token.kind == special_token!($s).into()
    }};
}
