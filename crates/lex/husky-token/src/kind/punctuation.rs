use husky_opn_syntax::{BinaryPunctuation, Bracket, SuffixPunctuation};

use crate::{Convexity, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Punctuation {
    Binary(BinaryPunctuation),
    Bra(Bracket),
    Ket(Bracket),
    Suffix(SuffixPunctuation),
    LAngle,            // <
    RAngle,            // >
    DeriveAssign,      // :=
    Minus,             // -
    DoubleVertical,    // ||
    BitNot,            // ~
    Dot,               // .
    Colon,             // :
    Comma,             // ,
    Ambersand,         // &
    Vertical,          // |
    Exclamation,       // !
    DoubleExclamation, // !!
    Semicolon,         // ;
    XmlKet,            // />
    At,                // @
    QuestionMark,      // ?
    PoundSign,         // #
}

impl From<BinaryPunctuation> for Punctuation {
    fn from(v: BinaryPunctuation) -> Self {
        Self::Binary(v)
    }
}

impl std::fmt::Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl Punctuation {
    pub fn code(&self) -> &'static str {
        match self {
            Punctuation::Binary(opr) => opr.code(),
            Punctuation::Bra(bra) => bra.bra_code(),
            Punctuation::Ket(ket) => ket.ket_code(),
            Punctuation::Suffix(_) => todo!(),
            Punctuation::LAngle => "<",
            Punctuation::RAngle => ">",
            Punctuation::DeriveAssign => ":=",
            Punctuation::Minus => "-",
            Punctuation::DoubleVertical => "||",
            Punctuation::BitNot => "~",
            Punctuation::Dot => ".",
            Punctuation::Colon => ":",
            Punctuation::Comma => ",",
            Punctuation::Ambersand => todo!(),
            Punctuation::Vertical => "|",
            Punctuation::Exclamation => "!",
            Punctuation::DoubleExclamation => "!!",
            Punctuation::Semicolon => ";",
            Punctuation::XmlKet => "/>",
            Punctuation::At => "@",
            Punctuation::QuestionMark => "?",
            Punctuation::PoundSign => "#",
        }
    }

    pub fn opt_bra(self) -> Option<Bracket> {
        match self {
            Punctuation::LAngle => Some(Bracket::Angle),
            Punctuation::Bra(bracket) => Some(bracket),
            _ => None,
        }
    }

    pub fn left_convexity(self) -> Option<Convexity> {
        match self {
            Punctuation::Binary(_) => todo!(),
            Punctuation::Bra(_) => todo!(),
            Punctuation::Ket(_) => todo!(),
            Punctuation::Suffix(_) => todo!(),
            Punctuation::LAngle => todo!(),
            Punctuation::RAngle => todo!(),
            Punctuation::DeriveAssign => todo!(),
            Punctuation::Minus => todo!(),
            Punctuation::DoubleVertical => todo!(),
            Punctuation::BitNot => todo!(),
            Punctuation::Dot => todo!(),
            Punctuation::Colon => todo!(),
            Punctuation::Comma => todo!(),
            Punctuation::Ambersand => todo!(),
            Punctuation::Vertical => todo!(),
            Punctuation::Exclamation => todo!(),
            Punctuation::DoubleExclamation => todo!(),
            Punctuation::Semicolon => todo!(),
            Punctuation::XmlKet => todo!(),
            Punctuation::At => todo!(),
            Punctuation::QuestionMark => todo!(),
            Punctuation::PoundSign => todo!(),
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

impl const From<Punctuation> for TokenKind {
    fn from(special: Punctuation) -> Self {
        TokenKind::Punctuation(special)
    }
}
