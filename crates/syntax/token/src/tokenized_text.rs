use std::sync::Arc;

use file::URange;
use word::WordAllocator;

use fold::FoldedList;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
    token_groups: FoldedList<URange>,
}

impl TokenizedText {
    pub fn new(
        line_groups: Vec<TokenGroup>,
        tokens: Vec<Token>,
        errors: Vec<LexError>,
    ) -> TokenizedText {
        TokenizedText {
            tokens,
            errors,
            token_groups: line_groups.into(),
        }
    }
}

pub type TokenGroupIter<'a> = fold::FoldIter<'a, [Token], TokenizedText>;

impl fold::FoldStorage<[Token]> for TokenizedText {
    fn len(&self) -> usize {
        self.token_groups.len()
    }

    fn next_sibling(&self, index: usize) -> Option<usize> {
        self.token_groups.next_sibling(index)
    }

    fn value(&self, index: usize) -> &[Token] {
        &self.tokens[self.token_groups.value(index).clone()]
    }

    fn this(&self) -> &TokenizedText {
        self
    }

    fn indent(&self, index: usize) -> fold::Indent {
        self.token_groups.indent(index)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroup {
    pub(crate) indent: TextIndent,
    pub(crate) tokens: URange,
}

impl fold::ItemToFold<URange> for TokenGroup {
    fn value(&self) -> std::ops::Range<usize> {
        self.tokens.clone()
    }

    fn indent(&self) -> fold::Indent {
        self.indent.raw
    }
}

impl TokenizedText {
    pub(crate) fn parse(word_unique_allocator: &WordAllocator, text: &str) -> Arc<Self> {
        let mut token_scanner = TokenScanner::new(word_unique_allocator);
        text.lines()
            .enumerate()
            .for_each(|(i, line)| token_scanner.scan(i, line));
        token_scanner.gen_tokenized_text()
    }
}
