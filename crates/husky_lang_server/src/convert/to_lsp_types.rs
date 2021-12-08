//! Conversion of husky-lang-server specific types to lsp_types equivalents.
use std::path;

use common::*;

use ide::{Fold, FoldKind};
use itertools::Itertools;

use crate::{line_collection::LineCollection, lsp_ext};

pub(crate) fn position(
    line_index: &husky_lang_db::line_map::LineMap,
    offset: TextSize,
) -> lsp_types::Position {
    let line_col = line_index.line_col(offset);
    lsp_types::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(
    line_index: &husky_lang_db::line_map::LineMap,
    range: TextRange,
) -> lsp_types::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp_types::Range::new(start, end)
}

pub(crate) fn to_diagnostic_severity(severity: hir::Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        hir::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        hir::Severity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
    }
}

pub(crate) fn to_diagnostic(_diagnostic: hir::Diagnostic) -> lsp_types::Diagnostic {
    todo!()
}

pub(crate) fn folding_range(
    text: &str,
    line_index: &husky_lang_db::line_map::LineMap,
    line_folding_only: bool,
    fold: Fold,
) -> lsp_types::FoldingRange {
    let kind = match fold.kind {
        FoldKind::Comment => Some(lsp_types::FoldingRangeKind::Comment),
        FoldKind::Imports => Some(lsp_types::FoldingRangeKind::Imports),
        FoldKind::Region => Some(lsp_types::FoldingRangeKind::Region),
        FoldKind::Mods
        | FoldKind::Block
        | FoldKind::ArgList
        | FoldKind::Consts
        | FoldKind::Statics
        | FoldKind::WhereClause
        | FoldKind::ReturnType
        | FoldKind::Array => None,
    };

    let range = range(line_index, fold.range);

    if line_folding_only {
        // Clients with line_folding_only == true (such as VSCode) will fold the whole end line
        // even if it contains text not in the folding range. To prevent that we exclude
        // range.end.line from the folding region if there is more text after range.end
        // on the same line.
        let has_more_text_on_end_line = text[TextRange::new(fold.range.end(), TextSize::of(text))]
            .chars()
            .take_while(|it| *it != '\n')
            .any(|it| !it.is_whitespace());

        let end_line = if has_more_text_on_end_line {
            range.end.line.saturating_sub(1)
        } else {
            range.end.line
        };

        lsp_types::FoldingRange {
            start_line: range.start.line,
            start_character: None,
            end_line,
            end_character: None,
            kind,
        }
    } else {
        lsp_types::FoldingRange {
            start_line: range.start.line,
            start_character: Some(range.start.character),
            end_line: range.end.line,
            end_character: Some(range.end.character),
            kind,
        }
    }
}

/// Returns a `Url` object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as `Url::from_file_path`.
pub(crate) fn url_from_abs_path(path: &std::path::Path) -> lsp_types::Url {
    todo!()
    // let url = lsp_types::Url::from_file_path(path).unwrap();
    // match path.as_ref().components().next() {
    //     Some(path::Component::Prefix(prefix))
    //         if matches!(
    //             prefix.kind(),
    //             path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
    //         ) =>
    //     {
    //         // Need to lowercase driver letter
    //     }
    //     _ => return url,
    // }

    // let driver_letter_range = {
    //     let (scheme, drive_letter, _rest) = match url.as_str().splitn(3, ':').collect_tuple() {
    //         Some(it) => it,
    //         None => return url,
    //     };
    //     let start = scheme.len() + ':'.len_utf8();
    //     start..(start + drive_letter.len())
    // };

    // // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // // machinery *also* canonicalizes the drive letter. So, just massage the
    // // string in place.
    // let mut url: String = url.into();
    // url[driver_letter_range].make_ascii_lowercase();
    // lsp_types::Url::parse(&url).unwrap()
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
