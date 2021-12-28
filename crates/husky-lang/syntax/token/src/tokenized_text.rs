use word::WordInterner;

use folded::FoldedList;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
    errors: Vec<LexError>,
    token_groups: FoldedList<Range>,
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

pub type TokenGroupIter<'a> = folded::FoldedIter<'a, [Token], TokenizedText>;

impl folded::FoldedContainer<[Token]> for TokenizedText {
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

    fn indent(&self, index: usize) -> folded::Indent {
        self.token_groups.indent(index)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroup {
    pub(crate) indent: Indent,
    pub(crate) tokens: Range,
}

impl folded::ItemToFold<Range> for TokenGroup {
    fn value(&self) -> Range {
        self.tokens.clone()
    }

    fn indent(&self) -> u16 {
        self.indent.into()
    }
}

impl TokenizedText {
    pub(crate) fn parse(word_interner: &WordInterner, text: &str) -> Self {
        let mut token_scanner = TokenScanner::new(word_interner);
        text.lines()
            .enumerate()
            .for_each(|(i, line)| token_scanner.scan(i, line));
        token_scanner.into()
    }
}
