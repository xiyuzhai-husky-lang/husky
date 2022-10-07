use lsp_types::{HoverContents, MarkupContent, MarkupKind};

pub trait HoverContentsQuery {
    fn hover_contents(&self) -> Option<HoverContents> {
        Some(HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: [
                "# Cohomology theories",
                "Michael Jordan is Goat",
                "```typescript",
                "husky.is_happy();",
                "```",
            ]
            .join("\n"),
        }))
    }
}
