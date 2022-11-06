use crate::*;
use husky_ast::RawExprVariant;
use husky_print_utils::ep;
use husky_term_infer::TermInferDb;
use husky_text::{FilePosition, FileRange, RangeInfo, TextRanged};
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use serde::{Deserialize, Serialize};

pub trait HoverDb: TermInferDb {
    fn opt_hover_result(&self, frange: FileRange) -> Option<HoverResult> {
        let entity_route_sheet = self.term_sheet(frange.file()).expect("todo");
        let (idx, expr) = entity_route_sheet
            .ast_text
            .find_first_expr_with_end_after(frange.text_start())?;
        Some(HoverResult {
            hover: lsp_types::Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!(r#"{expr:?}"#),
                }),
                range: Some(frange.text_range().into()),
            },
            actions: vec![
                CommandLinkGroup {
                    title: Some(format!("FirstCommandLinkeGroup")),
                    commands: vec![CommandLink {
                        command: lsp_types::Command {
                            command: format!("FirstCommandLinkeGroupFirstCommand"),
                            title: format!("FirstCommand"),
                            arguments: None,
                        },
                        tooltip: Some(format!("tooltip")),
                    }],
                },
                CommandLinkGroup::new_goto_types(),
            ],
        })
    }

    fn goto_implementation(
        &self,
        position: FilePosition,
    ) -> Option<RangeInfo<Vec<NavigationTarget>>> {
        unimplemented!()
    }

    fn hover_config(&self) -> &HoverConfig;
}
