use crate::{
    level::NamAstLevel,
    paragraph::{NamParagraph, NamParagraphIter},
};
use std::iter::Peekable;

pub struct NamParser<'a> {
    input: &'a str,
    paragraph_iter: Peekable<NamParagraphIter<'a>>,
    level: NamAstLevel,
}

impl<'a> NamParser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            input,
            paragraph_iter: NamParagraphIter::new(input).peekable(),
            level: NamAstLevel::Book,
        }
    }

    pub(crate) fn with_level<R>(&mut self, level: NamAstLevel, f: impl Fn(&mut Self) -> R) -> R {
        let prev_level = std::mem::replace(&mut self.level, level);
        let r = f(self);
        self.level = prev_level;
        r
    }

    pub(crate) fn next_paragraph(&mut self) -> Option<NamParagraph> {
        let paragraph = self.paragraph_iter.peek()?;
        if paragraph.lead.level() <= self.level {
            return None;
        }
        self.paragraph_iter.next()
    }
}
