use crate::{group::produce_group_starts, *};
use husky_text::TextIndent;
use husky_word::WordDb;
use lsp_types::FoldingRange;
use std::fmt::Write;
use std::sync::Arc;

// pub(crate) fn gen_tokenized_text(mut self) -> Arc<TokenizedText> {
//     let line_groups = self.produce_line_groups();
//     Arc::new(TokenizedText::new(line_groups, self.tokens, self.errors))
// }

#[derive(Debug, PartialEq, Eq)]
pub struct TokenSheet {
    tokens: Vec<Token>,
    line_group_starts: Vec<usize>,
}

impl TokenSheet {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn token_groups<'a>(&'a self) -> TokenGroupIter<'a> {
        TokenGroupIter::new(&self.tokens, &self.line_group_starts)
    }

    pub fn new(tokens: Vec<Token>) -> TokenSheet {
        TokenSheet {
            line_group_starts: produce_group_starts(&tokens),
            tokens,
        }
    }

    pub fn folding_ranges(&self) -> Vec<FoldingRange> {
        todo!()
        // let result = self
        //     .token_groups
        //     .nodes
        //     .iter()
        //     .filter_map(|node| {
        //         let start = self.tokens[node.value.start].range.start;
        //         let end = match node.folding_end {
        //             FoldingEnd::Sibling(idx) => {
        //                 let last_node = &self.token_groups.nodes[idx - 1];
        //                 self.tokens[last_node.value.end - 1].range.end
        //             }
        //             FoldingEnd::Elder(idx) => {
        //                 let last_node = &self.token_groups.nodes[idx - 1];
        //                 self.tokens[last_node.value.end - 1].range.end
        //             }
        //             FoldingEnd::EOF => self.tokens.last().unwrap().range.end,
        //         };
        //         Some(FoldingRange {
        //             start_line: start.i(),
        //             start_character: Some(start.j()),
        //             end_line: end.i(),
        //             end_character: Some(end.j()),
        //             kind: None,
        //         })
        //     })
        //     .collect();
        // result
    }

    pub fn summarize(&self) -> String {
        todo!()
        // let mut summary = String::new();
        // for (i, folded_result) in self.token_groups.nodes.iter().enumerate() {
        //     write!(
        //         summary,
        //         "#{},{} {:?}, {:?}\n",
        //         i,
        //         &((0..folded_result.indent)
        //             .into_iter()
        //             .map(|_| ' ')
        //             .collect::<String>()),
        //         folded_result.folding_end,
        //         folded_result.value
        //     )
        //     .unwrap();
        // }
        // summary
    }
}

impl TokenSheet {
    pub(crate) fn parse(db: &dyn WordDb, text: &str) -> Arc<Self> {
        todo!()
        // let mut token_scanner = TokenScanner::new(word_interner);
        // for (i, line) in text.lines().enumerate() {
        //     token_scanner.scan_line(i, line)
        // }
        // token_scanner.gen_tokenized_text()
    }
}
