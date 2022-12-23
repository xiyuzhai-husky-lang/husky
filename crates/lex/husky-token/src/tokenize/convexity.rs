use crate::{SpecialToken, Token, TokenKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Convexity {
    Convex,
    Concave,
    Any,
}
impl Convexity {
    pub fn compatible_with(self: Convexity, right: Convexity) -> bool {
        match self {
            Convexity::Convex => match right {
                Convexity::Convex => false,
                Convexity::Concave => true,
                Convexity::Any => true,
            },
            Convexity::Concave => match right {
                Convexity::Convex => true,
                Convexity::Concave => false,
                Convexity::Any => true,
            },
            Convexity::Any => true,
        }
    }
}

impl Token {
    pub fn left_convexity(self) -> Option<Convexity> {
        match self.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => Some(Convexity::Convex),
            TokenKind::Special(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Some(Convexity::Convex),
            TokenKind::Unrecognized(_) => None,
            TokenKind::IllFormedLiteral(_) => Some(Convexity::Convex),
            TokenKind::Comment => todo!(),
        }
    }

    pub fn right_convexity(&self) -> Convexity {
        match self.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_) => Convexity::Concave,
            TokenKind::Identifier(_) => Convexity::Convex,
            TokenKind::Special(special) => match special {
                SpecialToken::BinaryOpr(_) => Convexity::Concave,
                SpecialToken::Bra(_) => Convexity::Concave,
                SpecialToken::Ket(_) => Convexity::Convex,
                SpecialToken::LAngle => Convexity::Concave,
                SpecialToken::RAngle => Convexity::Any,
                SpecialToken::DeriveAssign => todo!(),
                SpecialToken::Minus => todo!(),
                SpecialToken::DoubleVertical => todo!(),
                SpecialToken::BitNot => todo!(),
                SpecialToken::FieldAccess => Convexity::Concave,
                SpecialToken::Colon => Convexity::Any,
                SpecialToken::Comma => Convexity::Concave,
                SpecialToken::Ambersand => todo!(),
                SpecialToken::Incr => todo!(),
                SpecialToken::Decr => todo!(),
                SpecialToken::Vertical => todo!(),
                SpecialToken::Exclamation => todo!(),
                SpecialToken::DoubleExclamation => todo!(),
                SpecialToken::Semicolon => todo!(),
                SpecialToken::XmlKet => todo!(),
                SpecialToken::At => todo!(),
                SpecialToken::QuestionMark => todo!(),
                SpecialToken::PoundSign => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Convexity::Convex,
            TokenKind::Unrecognized(_) => Convexity::Any,
            TokenKind::IllFormedLiteral(_) => Convexity::Convex,
            TokenKind::Comment => todo!(),
        }
    }
}
