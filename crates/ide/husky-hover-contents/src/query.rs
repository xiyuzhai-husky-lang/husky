use lsp_types::HoverContents;

pub trait HoverContentsQuery {
    fn hover_content(&self) -> HoverContents {
        todo!()
    }
}
