use crate::{Punctuation, Token, WordOpr};

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
            Token::Keyword(_) => Convexity::Concave,
            Token::Ident(_) | Token::Label(_) => Convexity::Convex,
            Token::Punctuation(punctuation) => punctuation.right_convexity(),
            Token::WordOpr(opr) => match opr {
                WordOpr::And => Convexity::Concave,
                WordOpr::Or => Convexity::Concave,
                WordOpr::As => Convexity::Concave,
                WordOpr::Be => Convexity::Concave,
            },
            Token::Literal(_) => Convexity::Convex,
            Token::Error(_) => Convexity::Any,
        }
    }
}
