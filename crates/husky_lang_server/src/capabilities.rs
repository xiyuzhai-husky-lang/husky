//! Advertises the capabilities of the LSP Server.
use lsp_types::ClientCapabilities;

/// Parses client capabilities and returns all completion resolve capabilities husky-lang-server supports.
pub(crate) fn completion_item_edit_resolve(caps: &ClientCapabilities) -> bool {
    (|| {
        Some(
            caps.text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .resolve_support
                .as_ref()?
                .properties
                .iter()
                .any(|cap_string| cap_string.as_str() == "additionalTextEdits"),
        )
    })() == Some(true)
}
