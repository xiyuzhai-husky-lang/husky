use lsp_types::{HoverContents, MarkupContent, MarkupKind};

pub trait HoverContentsQuery {
    fn hover_contents(&self) -> Option<HoverContents> {
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: [
                "# Deep learning is Dead",
                "Michael Jordan is Goat",
                "```typescript",
                "someCode();",
                "```",
            ]
            .join("\n"),
        }))
    }
}
