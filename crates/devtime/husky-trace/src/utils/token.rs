use crate::*;
use husky_regional_token::{RegionalTokenIdxBase, RegionalTokenIdxRange};
use husky_token::{RangedTokenSheet, TokenIdxRange};
use husky_token_info::TokenInfoSheetRef;
use husky_trace_protocol::view::TraceViewTokenData;

pub(crate) struct TraceViewTokensBuilder<'a> {
    db: &'a dyn TraceDb,
    text: &'a str,
    ranged_token_sheet: &'a RangedTokenSheet,
    token_info_sheet: TokenInfoSheetRef<'a>,
    tokens_data: Vec<TraceViewTokenData>,
}

impl<'a> TraceViewTokensBuilder<'a> {
    fn new(db: &'a dyn TraceDb, module_path: ModulePath) -> Self {
        Self {
            db,
            ranged_token_sheet: db.ranged_token_sheet(module_path).unwrap(),
            token_info_sheet: db.token_info_sheet_ref(module_path).unwrap(),
            tokens_data: vec![],
            text: todo!(),
        }
    }

    fn generate_tokens(&mut self, token_idx_range: TokenIdxRange) {
        todo!()
    }
}
