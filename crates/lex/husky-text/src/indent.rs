use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct TextIndent {
    pub raw: u8,
}

impl From<u8> for TextIndent {
    fn from(j: u8) -> Self {
        TextIndent { raw: j }
    }
}

impl From<TextIndent> for u8 {
    fn from(val: TextIndent) -> Self {
        val.raw
    }
}

impl From<usize> for TextIndent {
    fn from(j: usize) -> Self {
        TextIndent {
            raw: j.try_into().expect("a line should be short!"),
        }
    }
}

impl From<&usize> for TextIndent {
    fn from(j: &usize) -> Self {
        TextIndent {
            raw: (*j).try_into().expect("a line should be short!"),
        }
    }
}

impl TextIndent {
    pub fn get_raw(&self) -> u8 {
        self.raw
    }
    pub fn empty() -> TextIndent {
        u8::MAX.into()
    }

    pub fn within(&self, other: TextIndent) -> Result<bool, IndentErrorKind> {
        if self.raw >= other.raw + 4 {
            Ok(true)
        } else if self.raw <= other.raw {
            Ok(false)
        } else {
            Err(IndentErrorKind::Inappropriate)
        }
    }

    pub fn child(&self) -> Self {
        (self.raw + 4).into()
    }
}

impl<'token_line> From<&mut CharIter<'token_line>> for TextIndent {
    fn from(char_iter: &mut CharIter<'token_line>) -> Self {
        loop {
            if let Some((j, c)) = char_iter.peek() {
                if *c == ' ' {
                    char_iter.next();
                } else {
                    break j.into();
                }
            } else {
                break TextIndent::empty();
            }
        }
    }
}

#[derive(Debug)]
pub enum IndentErrorKind {
    Inappropriate,
}
