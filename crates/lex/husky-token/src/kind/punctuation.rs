use crate::*;
use husky_opn_syntax::{
    BinaryClosedOpr, BinaryComparisonOpr, BinaryOpr, BinaryShiftOpr, BinaryShortcuitLogicOpr,
    Bracket, SuffixOpr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub struct Punctuation(PunctuationMapped);

impl Punctuation {
    pub fn mapped(self) -> PunctuationMapped {
        self.0
    }

    pub fn code(self) -> &'static str {
        self.0.code()
    }

    pub fn right_convexity(self) -> Convexity {
        match self.0 {
            PunctuationMapped::Binary(_) => Convexity::Concave,
            PunctuationMapped::Suffix(_) => Convexity::Convex,
            PunctuationMapped::Bra(_) => Convexity::Concave,
            PunctuationMapped::Ket(_) => Convexity::Convex,
            PunctuationMapped::LaOrLt => Convexity::Concave,
            PunctuationMapped::ColonColonLa => Convexity::Concave,
            PunctuationMapped::RaOrGt => Convexity::Any,
            PunctuationMapped::Shr => Convexity::Concave,
            PunctuationMapped::DeriveAssign => Convexity::Concave,
            PunctuationMapped::Minus => Convexity::Concave,
            PunctuationMapped::DoubleVertical => Convexity::Concave,
            PunctuationMapped::Tilde => Convexity::Concave,
            PunctuationMapped::Dot => Convexity::Concave,
            PunctuationMapped::Colon => Convexity::Any,
            PunctuationMapped::Comma => Convexity::Concave,
            PunctuationMapped::Vertical => Convexity::Any,
            PunctuationMapped::Exclamation => Convexity::Concave,
            PunctuationMapped::DoubleExclamation => Convexity::Concave,
            PunctuationMapped::Semicolon => Convexity::Any,
            PunctuationMapped::XmlKet => Convexity::Convex,
            PunctuationMapped::At => Convexity::Concave,
            PunctuationMapped::AtEq => Convexity::Concave,
            PunctuationMapped::Ambersand => Convexity::Concave,
            PunctuationMapped::Pound => Convexity::Concave,
            PunctuationMapped::Question => Convexity::Any,
            PunctuationMapped::DotDot => Convexity::Concave,
            PunctuationMapped::Star => Convexity::Concave,
            PunctuationMapped::Sheba => Convexity::Any,
            PunctuationMapped::Eq => Convexity::Concave,
            PunctuationMapped::ForAll => Convexity::Concave,
            PunctuationMapped::Exists => Convexity::Concave,
            _ => unreachable!(),
        }
    }

    /// `->`
    pub const THIN_ARROW: Self = Self(PunctuationMapped::Binary(BinaryOpr::Curry));
    /// `:`
    pub const COLON: Self = Self(PunctuationMapped::Colon);
    /// `::`
    pub const COLON_COLON: Self = Self(PunctuationMapped::Binary(BinaryOpr::ScopeResolution));
    /// `;`
    pub const SEMICOLON: Self = Self(PunctuationMapped::Semicolon);
    /// `(`
    pub const LPAR: Self = Self(PunctuationMapped::Bra(Bracket::Par));
    /// `[`
    pub const LBOX: Self = Self(PunctuationMapped::Bra(Bracket::Box));
    /// `{`
    pub const LCURL: Self = Self(PunctuationMapped::Bra(Bracket::Curl));
    /// `<`
    pub const LA_OR_LT: Self = Self(PunctuationMapped::LaOrLt);
    /// `::<`
    pub const COLON_COLON_LA: Self = Self(PunctuationMapped::ColonColonLa);
    /// `)`
    pub const RPAR: Self = Self(PunctuationMapped::Ket(Bracket::Par));
    /// `]`
    pub const RBOX: Self = Self(PunctuationMapped::Ket(Bracket::Box));
    /// `}`
    pub const RCURL: Self = Self(PunctuationMapped::Ket(Bracket::Curl));
    /// `>`
    pub const RA_OR_GT: Self = Self(PunctuationMapped::RaOrGt);
    /// `|`
    pub const VERTICAL: Self = Self(PunctuationMapped::Vertical);
    /// `||`
    pub const DOUBLE_VERTICAL: Self = Self(PunctuationMapped::DoubleVertical);
    /// `..`
    pub const DOT_DOT: Self = Self(PunctuationMapped::DotDot);
    /// `~`
    pub const TILDE: Self = Self(PunctuationMapped::Tilde);
    /// `%`
    pub const REM_EUCLID: Self = Self(PunctuationMapped::Binary(BinaryOpr::Closed(
        BinaryClosedOpr::RemEuclid,
    )));
    pub const BITOR: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::BitOr,
    )));
    pub const AMBERSAND: Self = Self(PunctuationMapped::Ambersand);
    /// `.`
    pub const DOT: Self = Self(PunctuationMapped::Dot);

    pub const ADD: Self = Self(PunctuationMapped::Binary(BinaryOpr::Closed(
        BinaryClosedOpr::Add,
    )));
    pub const SUB: Self = Self(PunctuationMapped::Binary(BinaryOpr::Closed(
        BinaryClosedOpr::Sub,
    )));
    pub const DIV: Self = Self(PunctuationMapped::Binary(BinaryOpr::Closed(
        BinaryClosedOpr::Div,
    )));
    pub const MINUS: Self = Self(PunctuationMapped::Minus);
    pub const STAR_STAR: Self = Self(PunctuationMapped::Binary(BinaryOpr::Closed(
        BinaryClosedOpr::Power,
    )));
    /// `=`
    pub const EQ: Self = Self(PunctuationMapped::Eq);
    pub const ADD_ASSIGN: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::Add,
    )));
    pub const SUB_ASSIGN: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::Sub,
    )));
    /// `*=`
    pub const MUL_ASSIGN: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::Mul,
    )));
    /// `*=`
    pub const BIT_AND_ASSIGN: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::BitAnd,
    )));
    /// `>>`
    pub const SHL: Self = Self(PunctuationMapped::Binary(BinaryOpr::Shift(
        BinaryShiftOpr::Shl,
    )));
    /// `>>`
    pub const SHR: Self = Self(PunctuationMapped::Shr);
    /// `/=`
    pub const DIV_ASSIGN: Self = Self(PunctuationMapped::Binary(BinaryOpr::AssignClosed(
        BinaryClosedOpr::Div,
    )));
    /// `==`
    pub const EQ_EQ: Self = Self(PunctuationMapped::Binary(BinaryOpr::Comparison(
        BinaryComparisonOpr::Eq,
    )));
    /// `!=`
    pub const NEQ: Self = Self(PunctuationMapped::Binary(BinaryOpr::Comparison(
        BinaryComparisonOpr::Neq,
    )));
    /// `>=`
    pub const LEQ: Self = Self(PunctuationMapped::Binary(BinaryOpr::Comparison(
        BinaryComparisonOpr::Leq,
    )));
    /// `>=`
    pub const GEQ: Self = Self(PunctuationMapped::Binary(BinaryOpr::Comparison(
        BinaryComparisonOpr::Geq,
    )));
    pub const INCR: Self = Self(PunctuationMapped::Suffix(SuffixOpr::Incr));
    pub const DECR: Self = Self(PunctuationMapped::Suffix(SuffixOpr::Decr));
    /// `!`
    pub const EXCLAMATION: Self = Self(PunctuationMapped::Exclamation);
    /// `!!`
    pub const DOUBLE_EXCLAMATION: Self = Self(PunctuationMapped::DoubleExclamation);
    /// `?`
    pub const QUESTION: Self = Self(PunctuationMapped::Question);
    /// `#`
    pub const POUND: Self = Self(PunctuationMapped::Pound);
    /// `∀`
    pub const FOR_ALL: Self = Self(PunctuationMapped::ForAll);
    /// `∃`
    pub const EXISTS: Self = Self(PunctuationMapped::Exists);
    /// `:=`
    pub const COLON_EQ: Self = Self(PunctuationMapped::DeriveAssign);
    /// `,`
    pub const COMMA: Self = Self(PunctuationMapped::Comma);
    /// `@=`
    pub const AT_EQ: Self = Self(PunctuationMapped::AtEq);
    /// `@`
    pub const AT: Self = Self(PunctuationMapped::At);
    /// `$`
    pub const SHEBA: Self = Self(PunctuationMapped::Sheba);
    /// `&&`
    pub const LOGIC_AND: Self = Self(PunctuationMapped::Binary(BinaryOpr::ShortCircuitLogic(
        BinaryShortcuitLogicOpr::And,
    )));
    pub const STAR: Self = Self(PunctuationMapped::Star);
    pub const XML_KET: Self = Self(PunctuationMapped::XmlKet);
}

impl std::fmt::Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

/// serve as cached
/// for punctuation that unambiguously maps to an opr,
/// we use that opr to represent it
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub enum PunctuationMapped {
    // predetermined
    Binary(BinaryOpr),
    Bra(Bracket),
    Ket(Bracket),
    Suffix(SuffixOpr),
    /// `=`
    ///
    /// assignment, attributes, various type definitions
    Eq,
    /// `<`
    LaOrLt, // <
    ColonColonLa, // ::<
    /// `>`, represents one of several cases:
    /// 1) right angle bracket
    /// 2) greater than
    RaOrGt,
    /// `>>` shift right
    Shr,
    DeriveAssign,      // :=
    Minus,             // -
    DoubleVertical,    // ||
    Dot,               // .
    DotDot,            // `.`
    Colon,             // `:`
    Star,              // `*`
    Comma,             // `,`
    Ambersand,         // `&`
    Vertical,          // `|`
    Exclamation,       // `!`
    DoubleExclamation, // `!!`
    Semicolon,         // `;`
    XmlKet,            // `/>`
    Sheba,             // $
    /// `@`
    At,
    /// `@=`
    AtEq,
    /// `?`
    Question,
    /// written as `#`
    Pound,
    /// `~`
    Tilde,
    /// `∀`
    ForAll,
    /// `∃`
    Exists,
}

impl PunctuationMapped {
    pub fn code(self) -> &'static str {
        match self {
            PunctuationMapped::Binary(opr) => opr.code(),
            PunctuationMapped::Bra(bra) => bra.bra_code(),
            PunctuationMapped::Ket(ket) => ket.ket_code(),
            PunctuationMapped::Suffix(_) => todo!(),
            PunctuationMapped::LaOrLt => "<",
            PunctuationMapped::ColonColonLa => "::<",
            PunctuationMapped::RaOrGt => ">",
            PunctuationMapped::Shr => ">>",
            PunctuationMapped::DeriveAssign => ":=",
            PunctuationMapped::Minus => "-",
            PunctuationMapped::DoubleVertical => "||",
            PunctuationMapped::Tilde => "~",
            PunctuationMapped::Dot => ".",
            PunctuationMapped::DotDot => "..",
            PunctuationMapped::Colon => ":",
            PunctuationMapped::Comma => ",",
            PunctuationMapped::Ambersand => todo!(),
            PunctuationMapped::Vertical => "|",
            PunctuationMapped::Exclamation => "!",
            PunctuationMapped::DoubleExclamation => "!!",
            PunctuationMapped::Semicolon => ";",
            PunctuationMapped::XmlKet => "/>",
            PunctuationMapped::At => "@",
            PunctuationMapped::AtEq => "@=",
            PunctuationMapped::Question => "?",
            PunctuationMapped::Pound => "#",
            PunctuationMapped::Star => "*",
            PunctuationMapped::Sheba => todo!(),
            PunctuationMapped::Eq => todo!(),
            PunctuationMapped::Tilde => todo!(),
            PunctuationMapped::ForAll => todo!(),
            PunctuationMapped::Exists => todo!(),
        }
    }

    pub fn opt_bra(self) -> Option<Bracket> {
        match self {
            PunctuationMapped::LaOrLt => Some(Bracket::Angle),
            PunctuationMapped::Bra(bracket) => Some(bracket),
            _ => None,
        }
    }

    pub fn left_convexity(self) -> Option<Convexity> {
        match self {
            PunctuationMapped::Binary(_) => todo!(),
            PunctuationMapped::Bra(_) => todo!(),
            PunctuationMapped::Ket(_) => todo!(),
            PunctuationMapped::Suffix(_) => todo!(),
            PunctuationMapped::LaOrLt => todo!(),
            PunctuationMapped::ColonColonLa => todo!(),
            PunctuationMapped::RaOrGt => todo!(),
            PunctuationMapped::Shr => todo!(),
            PunctuationMapped::DeriveAssign => todo!(),
            PunctuationMapped::Minus => todo!(),
            PunctuationMapped::DoubleVertical => todo!(),
            PunctuationMapped::Tilde => todo!(),
            PunctuationMapped::Dot => todo!(),
            PunctuationMapped::DotDot => todo!(),
            PunctuationMapped::Colon => todo!(),
            PunctuationMapped::Comma => todo!(),
            PunctuationMapped::Ambersand => todo!(),
            PunctuationMapped::Vertical => todo!(),
            PunctuationMapped::Exclamation => todo!(),
            PunctuationMapped::DoubleExclamation => todo!(),
            PunctuationMapped::Semicolon => todo!(),
            PunctuationMapped::XmlKet => todo!(),
            PunctuationMapped::At => todo!(),
            PunctuationMapped::AtEq => todo!(),
            PunctuationMapped::Question => todo!(),
            PunctuationMapped::Pound => todo!(),
            PunctuationMapped::Star => todo!(),
            PunctuationMapped::Sheba => todo!(),
            PunctuationMapped::Eq => todo!(),
            PunctuationMapped::Tilde => todo!(),
            PunctuationMapped::ForAll => todo!(),
            PunctuationMapped::Exists => todo!(),
        }
    }
}
