use super::*;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for LxMathDigit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(LxMathDigit::Zero),
            '1' => Ok(LxMathDigit::One),
            '2' => Ok(LxMathDigit::Two),
            '3' => Ok(LxMathDigit::Three),
            '4' => Ok(LxMathDigit::Four),
            '5' => Ok(LxMathDigit::Five),
            '6' => Ok(LxMathDigit::Six),
            '7' => Ok(LxMathDigit::Seven),
            '8' => Ok(LxMathDigit::Eight),
            '9' => Ok(LxMathDigit::Nine),
            _ => Err(()),
        }
    }
}
