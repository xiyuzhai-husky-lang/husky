use crate::*;
use husky_text_protocol::{
    position::{FilePosition, TextPosition},
    range::RangeInfo,
};
use husky_token::TokenDb;

pub trait HoverDb {
    fn hover_result(&self, module_path: ModulePath, position: TextPosition) -> Option<HoverResult>;

    fn goto_implementation(
        &self,
        _position: FilePosition,
    ) -> Option<RangeInfo<Vec<NavigationTarget>>> {
        unimplemented!()
    }

    fn hover_config(&self) -> HoverConfig;
}

impl HoverDb for ::salsa::Db {
    fn hover_result(&self, module_path: ModulePath, pos: TextPosition) -> Option<HoverResult> {
        let ranged_token_sheet = self.ranged_token_sheet(module_path);
        let Some(token_idx) = ranged_token_sheet.search_token_by_position(pos) else {
            return None;
        };
        calc_hover_result(self, module_path, token_idx)
    }

    fn hover_config(&self) -> HoverConfig {
        hover_config(self)
    }
}
