use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionResponse,
};

pub trait HuskyCompletionQuery {
    fn completion(&self) -> Option<CompletionResponse> {
        Some(mimic_completion_response())
    }
}

fn mimic_completion_response() -> CompletionResponse {
    CompletionResponse::Array(vec![
        CompletionItem {
            label: "haha".to_string(),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some("haha has no details".to_string()),
                description: Some(format!("description haha")),
            }),
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
        },
        CompletionItem {
            label: "haha_field".to_string(),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some("haha has no details".to_string()),
                description: Some(format!("description haha")),
            }),
            kind: Some(CompletionItemKind::FIELD),
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
        },
        CompletionItem {
            label: "haha_value".to_string(),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some("haha has no details".to_string()),
                description: Some(format!("description haha")),
            }),
            kind: Some(CompletionItemKind::VALUE),
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
        },
        CompletionItem {
            label: "haha_function".to_string(),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some("haha has no details".to_string()),
                description: Some(format!("description haha")),
            }),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            documentation: None,
            deprecated: Some(true),
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
        },
    ])
}
