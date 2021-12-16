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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TextPosition {
    row: Row,
    col: Column,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TextRange {
    start: TextPosition,
    end: TextPosition,
}

impl From<std::ops::Range<&TextRange>> for TextRange {
    fn from(ranges: std::ops::Range<&TextRange>) -> Self {
        Self {
            start: ranges.start.start.clone(),
            end: ranges.end.end.clone(),
        }
    }
}

impl std::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}..{:?}", self.start, self.end))
    }
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

impl Into<lsp_types::Range> for TextRange {
    fn into(self) -> lsp_types::Range {
        lsp_types::Range::new(self.start.into(), self.end.into())
    }
}

pub trait GetTextRange {
    fn get_text_range(&self) -> &TextRange;
}

impl<T> From<&[T]> for TextRange
where
    T: GetTextRange,
{
    fn from(slice: &[T]) -> Self {
        (slice[0].get_text_range()..slice.last().unwrap().get_text_range()).into()
    }
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
