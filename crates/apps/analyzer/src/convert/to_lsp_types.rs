//! Conversion of husky-lang-server specific types to lsp_types equivalents.
#![allow(warnings, dead_code)]
use std::{
    path::{self, Path},
    sync::Arc,
};

use lsp_types::SemanticToken;
use token::{AbsSemanticToken, SemanticTokenKind};

use crate::lsp_ext;

/// Returns a `Url` object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as `Url::from_file_path`.
pub(crate) fn url_from_abs_path(path: &Path) -> lsp_types::Url {
    let url = lsp_types::Url::from_file_path(path).unwrap();
    match path.components().next() {
        Some(path::Component::Prefix(prefix))
            if matches!(
                prefix.kind(),
                path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
            ) =>
        {
            // Need to lowercase driver letter
        }
        _ => return url,
    }

    let driver_letter_range = {
        use itertools::Itertools;

        let (scheme, drive_letter, _rest) = match url.as_str().splitn(3, ':').collect_tuple() {
            Some(it) => it,
            None => return url,
        };
        let start = scheme.len() + ':'.len_utf8();
        start..(start + drive_letter.len())
    };

    // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // machinery *also* canonicalizes the drive letter. So, just massage the
    // string in place.
    let mut url: String = url.into();
    url[driver_letter_range].make_ascii_lowercase();
    lsp_types::Url::parse(&url).unwrap()
}
impl From<lsp_ext::SnippetWorkspaceEdit> for lsp_types::WorkspaceEdit {
    fn from(snippet_workspace_edit: lsp_ext::SnippetWorkspaceEdit) -> lsp_types::WorkspaceEdit {
        lsp_types::WorkspaceEdit {
            changes: None,
            document_changes: snippet_workspace_edit.document_changes.map(|changes| {
                lsp_types::DocumentChanges::Operations(
                    changes
                        .into_iter()
                        .map(|change| match change {
                            lsp_ext::SnippetDocumentChangeOperation::Op(op) => {
                                lsp_types::DocumentChangeOperation::Op(op)
                            }
                            lsp_ext::SnippetDocumentChangeOperation::Edit(edit) => {
                                lsp_types::DocumentChangeOperation::Edit(
                                    lsp_types::TextDocumentEdit {
                                        text_document: edit.text_document,
                                        edits: edit.edits.into_iter().map(From::from).collect(),
                                    },
                                )
                            }
                        })
                        .collect(),
                )
            }),
            change_annotations: snippet_workspace_edit.change_annotations,
        }
    }
}

impl From<lsp_ext::SnippetTextEdit>
    for lsp_types::OneOf<lsp_types::TextEdit, lsp_types::AnnotatedTextEdit>
{
    fn from(
        lsp_ext::SnippetTextEdit {
            annotation_id,
            insert_text_format: _,
            new_text,
            range,
        }: lsp_ext::SnippetTextEdit,
    ) -> Self {
        match annotation_id {
            Some(annotation_id) => lsp_types::OneOf::Right(lsp_types::AnnotatedTextEdit {
                text_edit: lsp_types::TextEdit { range, new_text },
                annotation_id,
            }),
            None => lsp_types::OneOf::Left(lsp_types::TextEdit { range, new_text }),
        }
    }
}

pub(crate) fn to_semantic_tokens(abs_semantic_tokens: &[AbsSemanticToken]) -> Vec<SemanticToken> {
    let mut semantic_tokens = vec![];
    let mut last_line = 0;
    let mut last_start = 0;
    for abs_semantic_token in abs_semantic_tokens {
        let new_line = abs_semantic_token.range.start.i();
        let new_start = abs_semantic_token.range.start.j();
        let length = abs_semantic_token.range.end.j() - new_start;
        semantic_tokens.push(SemanticToken {
            delta_line: new_line - last_line,
            delta_start: if new_line > last_line {
                new_start
            } else {
                new_start - last_start
            },
            length,
            token_type: abs_semantic_token.token_type,
            token_modifiers_bitset: abs_semantic_token.token_modifiers_bitset,
        });
        last_line = new_start;
        last_start = new_start
    }
    semantic_tokens
}
