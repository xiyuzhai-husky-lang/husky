use word::WordInterner;

use folded::FoldedList;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
    errors: Vec<LexError>,
    folded_list: FoldedList<Range>,
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
            folded_list: line_groups.into(),
        }
    }
}

pub type TokenGroupIter<'a> = folded::FoldedIter<'a, Range, [Token], TokenizedText>;

impl folded::Folded<Range, [Token], TokenizedText> for TokenizedText {
    fn nodes(&self) -> &[folded::FoldedNode<Range>] {
        self.folded_list.nodes()
    }

    fn value(&self, key: &Range) -> &[Token] {
        &self.tokens[key.clone()]
    }

    fn this(&self) -> &TokenizedText {
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenGroup {
    pub(crate) indent: Indent,
    pub(crate) tokens: Range,
}

impl folded::ItemToFold<Range> for TokenGroup {
    fn key(&self) -> Range {
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
