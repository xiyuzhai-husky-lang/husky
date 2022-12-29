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
        let token_sheet = self.token_sheet(module_path)?;
        let token_idx = token_sheet.search_token_by_position(pos);
        todo!()
        // let entity_route_sheet = self.term_sheet(frange.file()).expect("todo");
        // let (idx, expr) = entity_route_sheet
        //     .ast_text()
        //     .find_first_expr_with_end_after(frange.text_start())?;
        // Some(HoverResult {
        //     hover: lsp_types::Hover {
        //         contents: HoverContents::Markup(MarkupContent {
        //             kind: MarkupKind::Markdown,
        //             value: format!(r#"{expr:?}"#),
        //         }),
        //         range: Some(frange.text_range().into()),
        //     },
        //     actions: vec![
        //         CommandLinkGroup {
        //             title: Some(format!("FirstCommandLinkeGroup")),
        //             commands: vec![CommandLink {
        //                 command: lsp_types::Command {
        //                     command: format!("FirstCommandLinkeGroupFirstCommand"),
        //                     title: format!("FirstCommand"),
        //                     arguments: None,
        //                 },
        //                 tooltip: Some(format!("tooltip")),
        //             }],
        //         },
        //         CommandLinkGroup::new_goto_types(),
        //     ],
        // })
    }

    fn hover_config(&self) -> &HoverConfig {
        todo!()
    }
}
