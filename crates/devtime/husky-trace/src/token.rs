use crate::*;
use husky_regional_token::{RegionalTokenIdxBase, RegionalTokenIdxRange};
use husky_text::{HasText, Text};
use husky_token::{RangedTokenSheet, TokenIdx, TokenIdxRange};
use husky_token_info::{TokenInfo, TokenInfoSheetRef, TokenInfoSource};
use husky_trace_protocol::view::{SeparationAfter, TraceViewTokenData};

#[derive(Debug, PartialEq, Eq)]
pub struct TraceViewTokens {
    data: Vec<TraceViewTokenData>,
    sources: Vec<Option<TokenInfoSource>>,
}

impl TraceViewTokens {
    pub fn data(&self) -> &[TraceViewTokenData] {
        self.data.as_ref()
    }

    pub(crate) fn new(
        module_path: ModulePath,
        token_idx_range: TokenIdxRange,
        db: &dyn TraceDb,
    ) -> Self {
        let mut builder = TraceViewTokensBuilder::new(db, module_path);
        builder.generate_tokens(token_idx_range);
        builder.finish()
    }
}

struct TraceViewTokensBuilder<'a> {
    db: &'a dyn TraceDb,
    text: Text<'a>,
    ranged_token_sheet: &'a RangedTokenSheet,
    token_info_sheet: TokenInfoSheetRef<'a>,
    tokens_data: Vec<TraceViewTokenData>,
    sources: Vec<Option<TokenInfoSource>>,
}

impl<'a> TraceViewTokensBuilder<'a> {
    fn new(db: &'a dyn TraceDb, module_path: ModulePath) -> Self {
        // db.text
        Self {
            db,
            ranged_token_sheet: db.ranged_token_sheet(module_path).unwrap(),
            token_info_sheet: db.token_info_sheet_ref(module_path).unwrap(),
            text: module_path.text(db),
            tokens_data: vec![],
            sources: vec![],
        }
    }

    fn generate_tokens(&mut self, token_idx_range: TokenIdxRange) {
        for token_idx in token_idx_range {
            self.generate_token(token_idx)
        }
    }

    fn generate_token(&mut self, token_idx: TokenIdx) {
        let text_range = self.ranged_token_sheet.token_text_range(token_idx);
        let text = self.text.text_within(text_range);
        let db = self.db;
        let (token_class, src) = match self.token_info_sheet[token_idx].as_ref() {
            Some(token_info) => (token_info.data().token_class(db), Some(token_info.src())),
            None => (
                self.ranged_token_sheet.token_sheet_data(db)[token_idx].default_token_class(),
                None,
            ),
        };
        // todo: handle inline comments
        let spaces_after = if token_idx.index() < self.ranged_token_sheet.len() - 1 {
            let next_text_range = self.ranged_token_sheet.token_text_range(token_idx + 1);
            if next_text_range.start.line == text_range.end.line {
                SeparationAfter::SameLine {
                    spaces: next_text_range.start.col.0 - text_range.end.col.0,
                }
            } else {
                SeparationAfter::NextLine {
                    indent: next_text_range.start.col.0,
                }
            }
        } else {
            SeparationAfter::Eof
        };
        self.tokens_data.push(TraceViewTokenData::new(
            text.to_string(),
            token_class,
            spaces_after,
            /* ad hoc */ false,
        ));
        self.sources.push(src)
    }

    fn finish(self) -> TraceViewTokens {
        TraceViewTokens {
            data: self.tokens_data,
            sources: self.sources,
        }
    }
}
