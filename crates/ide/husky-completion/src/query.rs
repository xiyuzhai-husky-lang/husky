use lsp_types::{CompletionItem, CompletionResponse};

pub trait HuskyCompletionQuery {
    fn completion(&self) -> Option<CompletionResponse> {
        Some(mimic_completion_response())
    }
}

fn mimic_completion_response() -> CompletionResponse {
    CompletionResponse::Array(vec![CompletionItem {
        label: "haha".to_string(),
        label_details: None,
        kind: None,
        detail: None,
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }])
}
