use crate::*;

use husky_entity_tree::EntityTreeResult;
use husky_text::{FilePosition, ModuleRange, RangeInfo, TextPosition, TextRange};
use husky_token_info::TokenInfoDb;

pub trait HoverDb: salsa::DbWithJar<HoverJar> + TokenInfoDb {
    fn hover_result(
        &self,
        module_path: ModulePath,
        position: TextPosition,
    ) -> EntityTreeResult<Option<HoverResult>>;

    fn goto_implementation(
        &self,
        _position: FilePosition,
    ) -> Option<RangeInfo<Vec<NavigationTarget>>> {
        unimplemented!()
    }

    fn hover_config(&self) -> &HoverConfig;
}

impl<Db> HoverDb for Db
where
    Db: salsa::DbWithJar<HoverJar> + TokenInfoDb,
{
    fn hover_result(
        &self,
        module_path: ModulePath,
        pos: TextPosition,
    ) -> EntityTreeResult<Option<HoverResult>> {
        let ranged_token_sheet = self.ranged_token_sheet(module_path)?;
        let Some(token_idx) = ranged_token_sheet.search_token_by_position(pos)
                else { return Ok(None) };
        calc_hover_result(self, module_path, token_idx)
    }

    fn hover_config(&self) -> &HoverConfig {
        todo!()
    }
}
