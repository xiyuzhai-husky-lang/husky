use crate::*;

pub struct DocumentChange {
    pub range: Option<TextRange>,
    pub text: String,
}

#[cfg(feature = "lsp_support")]
impl From<lsp_types::TextDocumentContentChangeEvent> for DocumentChange {
    fn from(change: lsp_types::TextDocumentContentChangeEvent) -> Self {
        Self {
            range: change.range.map(|range| range.into()),
            text: change.text,
        }
    }
}
