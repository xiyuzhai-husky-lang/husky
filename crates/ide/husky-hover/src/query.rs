use crate::*;
use husky_ast::RawExprVariant;
use husky_infer_entity_route::InferEntityRouteQueryGroup;
use husky_print_utils::ep;
use husky_text::{FileRange, TextRanged};
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use serde::{Deserialize, Serialize};

pub trait HuskyHoverContentsQuery: InferEntityRouteQueryGroup {
    fn opt_hover_result(&self, frange: FileRange) -> Option<HoverResult> {
        let entity_route_sheet = self.entity_route_sheet(frange.file()).expect("todo");
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
            actions: vec![CommandLinkGroup {
                title: Some(format!("title")),
                commands: vec![CommandLink {
                    command: lsp_types::Command {
                        command: format!("command"),
                        title: format!("title"),
                        arguments: None,
                    },
                    tooltip: Some(format!("tooltip")),
                }],
            }],
        })
    }
}
