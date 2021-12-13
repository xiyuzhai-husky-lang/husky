use std::num::NonZeroU16;
use std::{iter::Enumerate, str::Chars};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Row(NonZeroU16);
impl From<u32> for Row {
    fn from(raw: u32) -> Self {
        unsafe {
            Row(NonZeroU16::new_unchecked(
                <u32 as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}
impl From<usize> for Row {
    fn from(raw: usize) -> Self {
        unsafe {
            Row(NonZeroU16::new_unchecked(
                <usize as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Column(NonZeroU16);
impl From<u32> for Column {
    fn from(raw: u32) -> Self {
        unsafe {
            Column(NonZeroU16::new_unchecked(
                <u32 as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}
impl From<usize> for Column {
    fn from(raw: usize) -> Self {
        unsafe {
            Column(NonZeroU16::new_unchecked(
                <usize as TryInto<u16>>::try_into(raw).expect("success") + 1,
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TextPosition {
    row: Row,
    col: Column,
}

impl From<(usize, usize)> for TextPosition {
    fn from(pair: (usize, usize)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TextRange {
    start: TextPosition,
    end: TextPosition,
}

impl TextRange {
    pub fn new_same_line(i: usize, start: usize, end: usize) -> TextRange {
        ((i, start), (i, end)).into()
    }
}

impl From<((usize, usize), (usize, usize))> for TextRange {
    fn from(pair: ((usize, usize), (usize, usize))) -> Self {
        TextRange {
            start: pair.0.into(),
            end: pair.1.into(),
        }
    }
}
pub trait GetTextRange {
    fn get_text_range(&self) -> &TextRange;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Indent {
    raw: u16,
}

impl From<u16> for Indent {
    fn from(j: u16) -> Self {
        Indent { raw: j }
    }
}

impl From<usize> for Indent {
    fn from(j: usize) -> Self {
        Indent {
            raw: j.try_into().expect("a line should be short!"),
        }
    }
}

impl From<&usize> for Indent {
    fn from(j: &usize) -> Self {
        Indent {
            raw: (*j).try_into().expect("a line should be short!"),
        }
    }
}

impl Indent {
    pub fn empty() -> Indent {
        u16::MAX.into()
    }

    pub fn within(&self, other: Indent) -> Result<bool, IndentErrorKind> {
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

pub type CharIter<'token_line> = std::iter::Peekable<Enumerate<Chars<'token_line>>>;

impl<'token_line> From<&mut CharIter<'token_line>> for Indent {
    fn from(char_iter: &mut CharIter<'token_line>) -> Self {
        loop {
            if let Some((j, c)) = char_iter.peek() {
                if *c == ' ' {
                    char_iter.next();
                } else {
                    break j.into();
                }
            } else {
                break Indent::empty();
            }
        }
    }
}

#[derive(Debug)]
pub enum IndentErrorKind {
    Inappropriate,
}
