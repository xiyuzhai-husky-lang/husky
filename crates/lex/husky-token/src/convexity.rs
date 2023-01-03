use crate::{Punctuation, Token, TokenKind};

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
    pub fn left_convexity(&self) -> Option<Convexity> {
        match self.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_) => todo!(),
            TokenKind::Identifier(_) => Some(Convexity::Convex),
            TokenKind::Punctuation(punc) => punc.left_convexity(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Some(Convexity::Convex),
            TokenKind::Comment => todo!(),
            TokenKind::Err(_) => unreachable!(),
        }
    }

    pub fn right_convexity(&self) -> Convexity {
        match self.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_) => Convexity::Concave,
            TokenKind::Identifier(_) => Convexity::Convex,
            TokenKind::Punctuation(special) => match special {
                Punctuation::Binary(_) => Convexity::Concave,
                Punctuation::Suffix(_) => todo!(),
                Punctuation::Bra(_) => Convexity::Concave,
                Punctuation::Ket(_) => Convexity::Convex,
                Punctuation::LAngle => Convexity::Concave,
                Punctuation::RAngle => Convexity::Any,
                Punctuation::DeriveAssign => todo!(),
                Punctuation::Minus => todo!(),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::BitNot => todo!(),
                Punctuation::Dot => Convexity::Concave,
                Punctuation::Colon => Convexity::Any,
                Punctuation::Comma => Convexity::Concave,
                Punctuation::Vertical => todo!(),
                Punctuation::Exclamation => todo!(),
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => todo!(),
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::Ambersand => todo!(),
                Punctuation::PoundSign => todo!(),
                Punctuation::Question => todo!(),
                Punctuation::DotDot => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(_) => Convexity::Convex,
            TokenKind::Comment => todo!(),
            TokenKind::Err(_) => unreachable!(),
        }
    }
}
