use husky_opn_syntax::{BinaryOpr, Bracket};

use crate::TokenKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpecialToken {
    BinaryOpr(BinaryOpr),
    Bra(Bracket),
    Ket(Bracket),
    LAngle,            // <
    RAngle,            // >
    DeriveAssign,      // ?=
    Minus,             // -
    DoubleVertical,    // ||
    BitNot,            // ~
    Dot,               // .
    Colon,             // :
    Comma,             // ,
    Ambersand,         // &
    Incr,              // ++
    Decr,              // --
    Vertical,          // |
    Exclamation,       // !
    DoubleExclamation, // !!
    Semicolon,         // ;
    XmlKet,            // />
    At,                // @
    QuestionMark,      // ?
    PoundSign,         // #
}

impl From<BinaryOpr> for SpecialToken {
    fn from(v: BinaryOpr) -> Self {
        Self::BinaryOpr(v)
    }
}

impl std::fmt::Display for SpecialToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl SpecialToken {
    pub fn code(&self) -> &'static str {
        match self {
            SpecialToken::BinaryOpr(opr) => opr.code(),
            SpecialToken::Bra(bra) => bra.bra_code(),
            SpecialToken::Ket(ket) => ket.ket_code(),
            SpecialToken::LAngle => "<",
            SpecialToken::RAngle => ">",
            SpecialToken::DeriveAssign => "?=",
            SpecialToken::Minus => "-",
            SpecialToken::DoubleVertical => "||",
            SpecialToken::BitNot => "~",
            SpecialToken::Dot => ".",
            SpecialToken::Colon => ":",
            SpecialToken::Comma => ",",
            SpecialToken::Ambersand => "&",
            SpecialToken::Incr => "++",
            SpecialToken::Decr => "--",
            SpecialToken::Vertical => "|",
            SpecialToken::Exclamation => "!",
            SpecialToken::DoubleExclamation => "!!",
            SpecialToken::Semicolon => ";",
            SpecialToken::XmlKet => "/>",
            SpecialToken::At => "@",
            SpecialToken::QuestionMark => "?",
            SpecialToken::PoundSign => "#",
        }
    }

    pub fn opt_bra(self) -> Option<Bracket> {
        match self {
            SpecialToken::LAngle => Some(Bracket::Angle),
            SpecialToken::Bra(bracket) => Some(bracket),
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
        SpecialToken::Bra(Bracket::Curl)
    }};
    ("}") => {{
        SpecialToken::Ket(Bracket::Curl)
    }};
    ("[") => {{
        SpecialToken::Bra(Bracket::Box)
    }};
    ("]") => {{
        SpecialToken::Ket(Bracket::Box)
    }};
    ("(") => {{
        SpecialToken::Bra(Bracket::Par)
    }};
    (")") => {{
        SpecialToken::Ket(Bracket::Par)
    }};
    ("+") => {{
        SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Add))
    }};
    ("-") => {{
        SpecialToken::SubOrMinus
    }};
    ("*") => {{
        SpecialToken::Star
    }};
    ("/") => {{
        SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Div))
    }};
    ("**") => {{
        SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Power))
    }};
    ("&&") => {{
        SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::And))
    }};
    ("||") => {{
        SpecialToken::DoubleVertical
    }};
    ("~") => {{
        SpecialToken::BitNot
    }};
    ("%") => {{
        SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid))
    }};
    (".") => {{
        SpecialToken::MemberAccess
    }};
    ("->") => {{
        SpecialToken::BinaryOpr(BinaryOpr::Curry)
    }};
    ("::") => {{
        SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)
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
        SpecialToken::BinaryOpr(BinaryOpr::Assign(None))
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

impl const From<SpecialToken> for TokenKind {
    fn from(special: SpecialToken) -> Self {
        TokenKind::Special(special)
    }
}
