use crate::*;

use husky_text::{FilePosition, FileRange, RangeInfo};

pub trait HoverDb {
    fn opt_hover_result(&self, _frange: FileRange) -> Option<HoverResult> {
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

    fn goto_implementation(
        &self,
        _position: FilePosition,
    ) -> Option<RangeInfo<Vec<NavigationTarget>>> {
        unimplemented!()
    }

    fn hover_config(&self) -> &HoverConfig;
}
