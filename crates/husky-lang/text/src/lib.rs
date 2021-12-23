use std::{iter::Enumerate, str::Chars};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Row(u32);
impl From<usize> for Row {
    fn from(raw: usize) -> Self {
        Row(<usize as TryInto<u32>>::try_into(raw).expect("success"))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Column(u32);
impl From<usize> for Column {
    fn from(raw: usize) -> Self {
        Column(<usize as TryInto<u32>>::try_into(raw).expect("success"))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TextPosition {
    row: Row,
    col: Column,
}

impl TextPosition {
    pub fn to_left(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 - x),
        }
    }

    pub fn to_right(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 + x),
        }
    }
}
impl std::fmt::Debug for TextPosition {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}:{:?}", self.row.0, self.col.0))
    }
}

impl From<(usize, usize)> for TextPosition {
    fn from(pair: (usize, usize)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl Into<lsp_types::Position> for TextPosition {
    fn into(self) -> lsp_types::Position {
        lsp_types::Position::new(self.row.0, self.col.0)
    }
}

pub type TextRange = std::ops::Range<TextPosition>;

pub fn new_same_line(i: usize, start: usize, end: usize) -> TextRange {
    ((i, start).into())..((i, end).into())
}

pub fn lsp_text_range(range: TextRange) -> lsp_types::Range {
    lsp_types::Range::new(range.start.into(), range.end.into())
}

pub trait HasTextRange {
    fn text_range_ref(&self) -> &TextRange;
    fn text_range(&self) -> TextRange {
        self.text_range_ref().clone()
    }
    fn text_range_to(&self, end: TextPosition) -> TextRange {
        self.text_range_ref().start..end
    }
    fn text_start(&self) -> TextPosition {
        self.text_range_ref().start
    }
    fn text_end(&self) -> TextPosition {
        self.text_range_ref().end
    }
    fn to(&self, range: &TextRange) -> TextRange {
        self.text_end()..range.end
    }
}

pub fn get_slice_text_range<T>(slice: &[T]) -> TextRange
where
    T: HasTextRange,
{
    (slice[0].text_range().start)..(slice.last().unwrap().text_range().end)
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

impl Into<u16> for Indent {
    fn into(self) -> u16 {
        self.raw
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
    pub fn get_raw(&self) -> u16 {
        self.raw
    }
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
