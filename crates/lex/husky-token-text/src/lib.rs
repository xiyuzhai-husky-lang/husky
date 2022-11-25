mod query;

use husky_identifier::IdentifierDb;
use husky_token_storage::TokenIdxRange;
pub use query::*;

use fold::{FoldableList, FoldingEnd};
use husky_text::TextIndent;
use husky_token::Token;
use husky_tokenize::LexError;
use lsp_types::FoldingRange;
use std::fmt::Write;
use std::sync::Arc;

// pub(crate) fn gen_tokenized_text(mut self) -> Arc<TokenizedText> {
//     let line_groups = self.produce_line_groups();
//     Arc::new(TokenizedText::new(line_groups, self.tokens, self.errors))
// }

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
    pub token_groups: FoldableList<TokenIdxRange>,
}

impl TokenizedText {
    pub fn new(
        line_groups: Vec<TokenGroup>,
        tokens: Vec<Token>,
        errors: Vec<LexError>,
    ) -> TokenizedText {
        todo!()
        // TokenizedText {
        //     tokens,
        //     errors,
        //     token_groups: line_groups.into(),
        // }
    }

    pub fn folding_ranges(&self) -> Vec<FoldingRange> {
        let result = self
            .token_groups
            .nodes
            .iter()
            .filter_map(|node| {
                let start = self.tokens[node.value.start].range.start;
                let end = match node.folding_end {
                    FoldingEnd::Sibling(idx) => {
                        let last_node = &self.token_groups.nodes[idx - 1];
                        self.tokens[last_node.value.end - 1].range.end
                    }
                    FoldingEnd::Elder(idx) => {
                        let last_node = &self.token_groups.nodes[idx - 1];
                        self.tokens[last_node.value.end - 1].range.end
                    }
                    FoldingEnd::EOF => self.tokens.last().unwrap().range.end,
                };
                Some(FoldingRange {
                    start_line: start.i(),
                    start_character: Some(start.j()),
                    end_line: end.i(),
                    end_character: Some(end.j()),
                    kind: None,
                })
            })
            .collect();
        result
    }

    pub fn summarize(&self) -> String {
        let mut summary = String::new();
        for (i, folded_result) in self.token_groups.nodes.iter().enumerate() {
            write!(
                summary,
                "#{},{} {:?}, {:?}\n",
                i,
                &((0..folded_result.indent)
                    .into_iter()
                    .map(|_| ' ')
                    .collect::<String>()),
                folded_result.folding_end,
                folded_result.value
            )
            .unwrap();
        }
        summary
    }
}

pub type TokenGroupIter<'a> = fold::FoldableIter<'a, TokenizedText>;

impl fold::FoldableStorage for TokenizedText {
    type Value = [Token];

    fn len(&self) -> usize {
        self.token_groups.len()
    }

    fn folding_end(&self, index: usize) -> FoldingEnd {
        self.token_groups.folding_end(index)
    }

    fn value(&self, index: usize) -> &[Token] {
        todo!()
        // &self.tokens[self.token_groups.value(index).clone()]
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
    pub indent: TextIndent,
    pub tokens: TokenIdxRange,
}

impl fold::ItemToFold<TokenIdxRange> for TokenGroup {
    fn value(&self) -> TokenIdxRange {
        self.tokens.clone()
    }

    fn indent(&self) -> fold::Indent {
        self.indent.raw
    }
}

impl TokenizedText {
    pub(crate) fn parse(db: &dyn IdentifierDb, text: &str) -> Arc<Self> {
        todo!()
        // let mut token_scanner = TokenScanner::new(word_interner);
        // for (i, line) in text.lines().enumerate() {
        //     token_scanner.scan_line(i, line)
        // }
        // token_scanner.gen_tokenized_text()
    }
}
