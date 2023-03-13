use crate::{Punctuation, Token};

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
    // pub fn left_convexity(&self) -> Option<Convexity> {
    //     match self {
    //         Token::Attr(_) => todo!(),
    //         Token::Keyword(_) => todo!(),
    //         Token::Ident(_) | Token::Label(_) => Some(Convexity::Convex),
    //         Token::Punctuation(punc) => punc.left_convexity(),
    //         Token::WordOpr(_) => todo!(),
    //         Token::Literal(_) => Some(Convexity::Convex),
    //         Token::Error(_) => unreachable!(),
    //     }
    // }

    pub fn right_convexity(&self) -> Convexity {
        match self {
            Token::Attr(_) => todo!(),
            Token::Keyword(_) => Convexity::Concave,
            Token::Ident(_) | Token::Label(_) => Convexity::Convex,
            Token::Punctuation(punctuation) => match punctuation {
                Punctuation::Binary(_) => Convexity::Concave,
                Punctuation::Suffix(_) => todo!(),
                Punctuation::Bra(_) => Convexity::Concave,
                Punctuation::Ket(_) => Convexity::Convex,
                Punctuation::LaOrLt => Convexity::Concave,
                Punctuation::ColonColonLAngle => todo!(),
                Punctuation::RaOrGt => Convexity::Any,
                Punctuation::Shr => todo!(),
                Punctuation::DeriveAssign => todo!(),
                Punctuation::Minus => todo!(),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::Tilde => todo!(),
                Punctuation::Dot => Convexity::Concave,
                Punctuation::Colon => Convexity::Any,
                Punctuation::ColonColon => todo!(),
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
                Punctuation::Star => todo!(),
                Punctuation::Sheba => Convexity::Any,
                Punctuation::Eq => Convexity::Concave,
                Punctuation::EqEq => todo!(),
                Punctuation::Tilde => todo!(),
                Punctuation::ForAll => todo!(),
                Punctuation::Exists => todo!(),
            },
            Token::WordOpr(_) => todo!(),
            Token::Literal(_) => Convexity::Convex,
            Token::Error(_) => Convexity::Any,
        }
    }
}
