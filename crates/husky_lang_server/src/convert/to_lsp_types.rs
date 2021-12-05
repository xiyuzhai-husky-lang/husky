//! Conversion of rust-analyzer specific types to lsp_types equivalents.
use std::{
    iter::once,
    path,
    sync::atomic::{AtomicU32, Ordering},
};

use ide::{
    Annotation, AnnotationKind, Assist, AssistKind, CallInfo, Cancellable, CompletionItem,
    CompletionItemKind, CompletionRelevance, Documentation, FileID, FileRange, FileSystemEdit,
    Fold, FoldKind, Highlight, HlMod, HlOperator, HlPunct, HlRange, HlTag, Indel, Markup,
    NavigationTarget, ReferenceCategory, RenameError, Severity, SourceChange, StructureNodeKind,
    SymbolKind, TextEdit, TextRange, TextSize,
};
use itertools::Itertools;
use serde_json::to_value;
use vfs::AbsPath;

use crate::{
    config::ServerConfig,
    line_index::{LineCollection, LineEndingType, OffsetEncoding},
    lsp_ext,
    lsp_utils::invalid_params_error,
    semantic_tokens,
    server_snapshot::ServerSnapshot,
    Result,
};

pub(crate) fn position(line_index: &LineCollection, offset: TextSize) -> lsp_types::Position {
    let line_col = line_index.begins.line_col(offset);
    lsp_types::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(line_index: &LineCollection, range: TextRange) -> lsp_types::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp_types::Range::new(start, end)
}

pub(crate) fn symbol_kind(symbol_kind: SymbolKind) -> lsp_types::SymbolKind {
    match symbol_kind {
        SymbolKind::Function => lsp_types::SymbolKind::FUNCTION,
        SymbolKind::Struct => lsp_types::SymbolKind::STRUCT,
        SymbolKind::Enum => lsp_types::SymbolKind::ENUM,
        SymbolKind::Variant => lsp_types::SymbolKind::ENUM_MEMBER,
        SymbolKind::Trait => lsp_types::SymbolKind::INTERFACE,
        SymbolKind::Macro => lsp_types::SymbolKind::FUNCTION,
        SymbolKind::Module => lsp_types::SymbolKind::MODULE,
        SymbolKind::TypeAlias | SymbolKind::TypeParam => lsp_types::SymbolKind::TYPE_PARAMETER,
        SymbolKind::Field => lsp_types::SymbolKind::FIELD,
        SymbolKind::Static => lsp_types::SymbolKind::CONSTANT,
        SymbolKind::Const => lsp_types::SymbolKind::CONSTANT,
        SymbolKind::ConstParam => lsp_types::SymbolKind::CONSTANT,
        SymbolKind::Impl => lsp_types::SymbolKind::OBJECT,
        SymbolKind::Local
        | SymbolKind::SelfParam
        | SymbolKind::LifetimeParam
        | SymbolKind::ValueParam
        | SymbolKind::Label => lsp_types::SymbolKind::VARIABLE,
        SymbolKind::Union => lsp_types::SymbolKind::STRUCT,
    }
}

pub(crate) fn structure_node_kind(kind: StructureNodeKind) -> lsp_types::SymbolKind {
    match kind {
        StructureNodeKind::SymbolKind(symbol) => symbol_kind(symbol),
        StructureNodeKind::Region => lsp_types::SymbolKind::NAMESPACE,
    }
}

pub(crate) fn document_highlight_kind(
    category: ReferenceCategory,
) -> lsp_types::DocumentHighlightKind {
    match category {
        ReferenceCategory::Read => lsp_types::DocumentHighlightKind::READ,
        ReferenceCategory::Write => lsp_types::DocumentHighlightKind::WRITE,
    }
}

pub(crate) fn diagnostic_severity(severity: Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        Severity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
    }
}

pub(crate) fn documentation(documentation: Documentation) -> lsp_types::Documentation {
    todo!()
}

pub(crate) fn completion_item_kind(
    completion_item_kind: CompletionItemKind,
) -> lsp_types::CompletionItemKind {
    match completion_item_kind {
        CompletionItemKind::Attribute => lsp_types::CompletionItemKind::ENUM_MEMBER,
        CompletionItemKind::Binding => lsp_types::CompletionItemKind::VARIABLE,
        CompletionItemKind::BuiltinType => lsp_types::CompletionItemKind::STRUCT,
        CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
        CompletionItemKind::Method => lsp_types::CompletionItemKind::METHOD,
        CompletionItemKind::Snippet => lsp_types::CompletionItemKind::SNIPPET,
        CompletionItemKind::UnresolvedReference => lsp_types::CompletionItemKind::REFERENCE,
        CompletionItemKind::SymbolKind(symbol) => match symbol {
            SymbolKind::Const => lsp_types::CompletionItemKind::CONSTANT,
            SymbolKind::ConstParam => lsp_types::CompletionItemKind::TYPE_PARAMETER,
            SymbolKind::Enum => lsp_types::CompletionItemKind::ENUM,
            SymbolKind::Field => lsp_types::CompletionItemKind::FIELD,
            SymbolKind::Function => lsp_types::CompletionItemKind::FUNCTION,
            SymbolKind::Impl => lsp_types::CompletionItemKind::TEXT,
            SymbolKind::Label => lsp_types::CompletionItemKind::VARIABLE,
            SymbolKind::LifetimeParam => lsp_types::CompletionItemKind::TYPE_PARAMETER,
            SymbolKind::Local => lsp_types::CompletionItemKind::VARIABLE,
            SymbolKind::Macro => lsp_types::CompletionItemKind::METHOD,
            SymbolKind::Module => lsp_types::CompletionItemKind::MODULE,
            SymbolKind::SelfParam => lsp_types::CompletionItemKind::VALUE,
            SymbolKind::Static => lsp_types::CompletionItemKind::VALUE,
            SymbolKind::Struct => lsp_types::CompletionItemKind::STRUCT,
            SymbolKind::Trait => lsp_types::CompletionItemKind::INTERFACE,
            SymbolKind::TypeAlias => lsp_types::CompletionItemKind::STRUCT,
            SymbolKind::TypeParam => lsp_types::CompletionItemKind::TYPE_PARAMETER,
            SymbolKind::Union => lsp_types::CompletionItemKind::STRUCT,
            SymbolKind::ValueParam => lsp_types::CompletionItemKind::VALUE,
            SymbolKind::Variant => lsp_types::CompletionItemKind::ENUM_MEMBER,
        },
    }
}

pub(crate) fn text_edit(line_index: &LineCollection, indel: Indel) -> lsp_types::TextEdit {
    let range = range(line_index, indel.delete);
    let new_text = match line_index.ending_type {
        LineEndingType::Unix => indel.insert,
        LineEndingType::Dos => indel.insert.replace('\n', "\r\n"),
    };
    lsp_types::TextEdit { range, new_text }
}

pub(crate) fn completion_text_edit(
    line_index: &LineCollection,
    insert_replace_support: Option<lsp_types::Position>,
    indel: Indel,
) -> lsp_types::CompletionTextEdit {
    let text_edit = text_edit(line_index, indel);
    match insert_replace_support {
        Some(cursor_pos) => lsp_types::InsertReplaceEdit {
            new_text: text_edit.new_text,
            insert: lsp_types::Range {
                start: text_edit.range.start,
                end: cursor_pos,
            },
            replace: text_edit.range,
        }
        .into(),
        None => text_edit.into(),
    }
}

pub(crate) fn snippet_text_edit(
    line_index: &LineCollection,
    is_snippet: bool,
    indel: Indel,
) -> lsp_ext::SnippetTextEdit {
    let text_edit = text_edit(line_index, indel);
    let insert_text_format = if is_snippet {
        Some(lsp_types::InsertTextFormat::SNIPPET)
    } else {
        None
    };
    lsp_ext::SnippetTextEdit {
        range: text_edit.range,
        new_text: text_edit.new_text,
        insert_text_format,
        annotation_id: None,
    }
}

pub(crate) fn text_edit_vec(
    line_index: &LineCollection,
    text_edit: TextEdit,
) -> Vec<lsp_types::TextEdit> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(line_index, indel))
        .collect()
}

pub(crate) fn snippet_text_edit_vec(
    line_index: &LineCollection,
    is_snippet: bool,
    text_edit: TextEdit,
) -> Vec<lsp_ext::SnippetTextEdit> {
    text_edit
        .into_iter()
        .map(|indel| self::snippet_text_edit(line_index, is_snippet, indel))
        .collect()
}

pub(crate) fn completion_items(
    config: &ServerConfig,
    line_index: &LineCollection,
    tdpp: lsp_types::TextDocumentPositionParams,
    items: Vec<CompletionItem>,
) -> Vec<lsp_types::CompletionItem> {
    let max_relevance = items
        .iter()
        .map(|it| it.relevance().score())
        .max()
        .unwrap_or_default();
    let mut res = Vec::with_capacity(items.len());
    for item in items {
        completion_item(&mut res, config, line_index, &tdpp, max_relevance, item)
    }
    res
}

fn completion_item(
    acc: &mut Vec<lsp_types::CompletionItem>,
    config: &ServerConfig,
    line_index: &LineCollection,
    tdpp: &lsp_types::TextDocumentPositionParams,
    max_relevance: u32,
    item: CompletionItem,
) {
    todo!()
}

pub(crate) fn signature_help(
    call_info: CallInfo,
    concise: bool,
    label_offsets: bool,
) -> lsp_types::SignatureHelp {
    let (label, parameters) = match (concise, label_offsets) {
        (_, false) => {
            let params = call_info
                .parameter_labels()
                .map(|label| lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::Simple(label.to_string()),
                    documentation: None,
                })
                .collect::<Vec<_>>();
            let label = if concise {
                call_info.parameter_labels().join(", ")
            } else {
                call_info.signature
            };
            (label, params)
        }
        (false, true) => {
            let params = call_info
                .parameter_ranges()
                .iter()
                .map(|it| [u32::from(it.start()), u32::from(it.end())])
                .map(|label_offsets| lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::LabelOffsets(label_offsets),
                    documentation: None,
                })
                .collect::<Vec<_>>();
            (call_info.signature, params)
        }
        (true, true) => {
            let mut params = Vec::new();
            let mut label = String::new();
            let mut first = true;
            for param in call_info.parameter_labels() {
                if !first {
                    label.push_str(", ");
                }
                first = false;
                let start = label.len() as u32;
                label.push_str(param);
                let end = label.len() as u32;
                params.push(lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::LabelOffsets([start, end]),
                    documentation: None,
                });
            }

            (label, params)
        }
    };

    let documentation = if concise {
        None
    } else {
        call_info.doc.map(|doc| {
            lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: doc,
            })
        })
    };

    let active_parameter = call_info.active_parameter.map(|it| it as u32);

    let signature = lsp_types::SignatureInformation {
        label,
        documentation,
        parameters: Some(parameters),
        active_parameter,
    };
    lsp_types::SignatureHelp {
        signatures: vec![signature],
        active_signature: Some(0),
        active_parameter,
    }
}

static TOKEN_RESULT_COUNTER: AtomicU32 = AtomicU32::new(1);

pub(crate) fn to_semantic_tokens(
    text: &str,
    line_index: &LineCollection,
    highlights: Vec<HlRange>,
    highlight_strings: bool,
) -> lsp_types::SemanticTokens {
    let id = TOKEN_RESULT_COUNTER
        .fetch_add(1, Ordering::SeqCst)
        .to_string();
    let mut builder = semantic_tokens::SemanticTokensBuilder::new(id);

    for highlight_range in highlights {
        if highlight_range.highlight.is_empty() {
            continue;
        }
        let (ty, mods) = semantic_token_type_and_modifiers(highlight_range.highlight);
        if !highlight_strings && ty == lsp_types::SemanticTokenType::STRING {
            continue;
        }
        let token_index = semantic_tokens::get_type_index(ty);
        let modifier_bitset = mods.0;

        for mut text_range in line_index.begins.lines(highlight_range.range) {
            if text[text_range].ends_with('\n') {
                text_range =
                    TextRange::new(text_range.start(), text_range.end() - TextSize::of('\n'));
            }
            let range = range(line_index, text_range);
            builder.push(range, token_index, modifier_bitset);
        }
    }

    builder.build()
}

pub(crate) fn semantic_token_delta(
    previous: &lsp_types::SemanticTokens,
    current: &lsp_types::SemanticTokens,
) -> lsp_types::SemanticTokensDelta {
    todo!()
}

fn semantic_token_type_and_modifiers(
    highlight: Highlight,
) -> (lsp_types::SemanticTokenType, semantic_tokens::ModifierSet) {
    let mut mods = semantic_tokens::ModifierSet::default();
    let type_ = match highlight.tag {
        HlTag::Symbol(symbol) => match symbol {
            SymbolKind::Module => lsp_types::SemanticTokenType::NAMESPACE,
            SymbolKind::Impl => semantic_tokens::TYPE_ALIAS,
            SymbolKind::Field => lsp_types::SemanticTokenType::PROPERTY,
            SymbolKind::TypeParam => lsp_types::SemanticTokenType::TYPE_PARAMETER,
            SymbolKind::ConstParam => semantic_tokens::CONST_PARAMETER,
            SymbolKind::LifetimeParam => semantic_tokens::LIFETIME,
            SymbolKind::Label => semantic_tokens::LABEL,
            SymbolKind::ValueParam => lsp_types::SemanticTokenType::PARAMETER,
            SymbolKind::SelfParam => semantic_tokens::SELF_KEYWORD,
            SymbolKind::Local => lsp_types::SemanticTokenType::VARIABLE,
            SymbolKind::Function => {
                if highlight.mods.contains(HlMod::Associated) {
                    lsp_types::SemanticTokenType::METHOD
                } else {
                    lsp_types::SemanticTokenType::FUNCTION
                }
            }
            SymbolKind::Const => {
                mods |= semantic_tokens::CONSTANT;
                mods |= lsp_types::SemanticTokenModifier::STATIC;
                lsp_types::SemanticTokenType::VARIABLE
            }
            SymbolKind::Static => {
                mods |= lsp_types::SemanticTokenModifier::STATIC;
                lsp_types::SemanticTokenType::VARIABLE
            }
            SymbolKind::Struct => lsp_types::SemanticTokenType::STRUCT,
            SymbolKind::Enum => lsp_types::SemanticTokenType::ENUM,
            SymbolKind::Variant => lsp_types::SemanticTokenType::ENUM_MEMBER,
            SymbolKind::Union => semantic_tokens::UNION,
            SymbolKind::TypeAlias => semantic_tokens::TYPE_ALIAS,
            SymbolKind::Trait => lsp_types::SemanticTokenType::INTERFACE,
            SymbolKind::Macro => lsp_types::SemanticTokenType::MACRO,
        },
        HlTag::Attribute => semantic_tokens::ATTRIBUTE,
        HlTag::BoolLiteral => semantic_tokens::BOOLEAN,
        HlTag::BuiltinAttr => semantic_tokens::BUILTIN_ATTRIBUTE,
        HlTag::BuiltinType => semantic_tokens::BUILTIN_TYPE,
        HlTag::ByteLiteral | HlTag::NumericLiteral => lsp_types::SemanticTokenType::NUMBER,
        HlTag::CharLiteral => semantic_tokens::CHAR,
        HlTag::Comment => lsp_types::SemanticTokenType::COMMENT,
        HlTag::EscapeSequence => semantic_tokens::ESCAPE_SEQUENCE,
        HlTag::FormatSpecifier => semantic_tokens::FORMAT_SPECIFIER,
        HlTag::Keyword => lsp_types::SemanticTokenType::KEYWORD,
        HlTag::None => semantic_tokens::GENERIC,
        HlTag::Operator(op) => match op {
            HlOperator::Bitwise => semantic_tokens::BITWISE,
            HlOperator::Arithmetic => semantic_tokens::ARITHMETIC,
            HlOperator::Logical => semantic_tokens::LOGICAL,
            HlOperator::Comparison => semantic_tokens::COMPARISON,
            HlOperator::Other => semantic_tokens::OPERATOR,
        },
        HlTag::StringLiteral => lsp_types::SemanticTokenType::STRING,
        HlTag::UnresolvedReference => semantic_tokens::UNRESOLVED_REFERENCE,
        HlTag::Punctuation(punct) => match punct {
            HlPunct::Bracket => semantic_tokens::BRACKET,
            HlPunct::Brace => semantic_tokens::BRACE,
            HlPunct::Parenthesis => semantic_tokens::PARENTHESIS,
            HlPunct::Angle => semantic_tokens::ANGLE,
            HlPunct::Comma => semantic_tokens::COMMA,
            HlPunct::Dot => semantic_tokens::DOT,
            HlPunct::Colon => semantic_tokens::COLON,
            HlPunct::Semi => semantic_tokens::SEMICOLON,
            HlPunct::Other => semantic_tokens::PUNCTUATION,
        },
    };

    for modifier in highlight.mods.iter() {
        let modifier = match modifier {
            HlMod::Associated => continue,
            HlMod::Async => semantic_tokens::ASYNC,
            HlMod::Attribute => semantic_tokens::ATTRIBUTE_MODIFIER,
            HlMod::Callable => semantic_tokens::CALLABLE,
            HlMod::Consuming => semantic_tokens::CONSUMING,
            HlMod::ControlFlow => semantic_tokens::CONTROL_FLOW,
            HlMod::CrateRoot => semantic_tokens::CRATE_ROOT,
            HlMod::DefaultLibrary => lsp_types::SemanticTokenModifier::DEFAULT_LIBRARY,
            HlMod::Definition => lsp_types::SemanticTokenModifier::DECLARATION,
            HlMod::Documentation => lsp_types::SemanticTokenModifier::DOCUMENTATION,
            HlMod::Injected => semantic_tokens::INJECTED,
            HlMod::IntraDocLink => semantic_tokens::INTRA_DOC_LINK,
            HlMod::Library => semantic_tokens::LIBRARY,
            HlMod::Mutable => semantic_tokens::MUTABLE,
            HlMod::Public => semantic_tokens::PUBLIC,
            HlMod::Reference => semantic_tokens::REFERENCE,
            HlMod::Static => lsp_types::SemanticTokenModifier::STATIC,
            HlMod::Trait => semantic_tokens::TRAIT_MODIFIER,
            HlMod::Unsafe => semantic_tokens::UNSAFE,
        };
        mods |= modifier;
    }

    (type_, mods)
}

pub(crate) fn folding_range(
    text: &str,
    line_index: &LineCollection,
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

pub(crate) fn url(snap: &ServerSnapshot, file_id: FileID) -> lsp_types::Url {
    todo!()
}

/// Returns a `Url` object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as `Url::from_file_path`.
pub(crate) fn url_from_abs_path(path: &AbsPath) -> lsp_types::Url {
    let url = lsp_types::Url::from_file_path(path).unwrap();
    match path.as_ref().components().next() {
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

pub(crate) fn optional_versioned_text_document_identifier(
    snap: &ServerSnapshot,
    file_id: FileID,
) -> lsp_types::OptionalVersionedTextDocumentIdentifier {
    todo!()
}

pub(crate) fn location(snap: &ServerSnapshot, frange: FileRange) -> Result<lsp_types::Location> {
    let url = url(snap, frange.file_id);
    let line_index = snap.file_line_index(frange.file_id)?;
    let range = range(&line_index, frange.range);
    let loc = lsp_types::Location::new(url, range);
    Ok(loc)
}

/// Prefer using `location_link`, if the client has the cap.
pub(crate) fn location_from_nav(
    snap: &ServerSnapshot,
    nav: NavigationTarget,
) -> Result<lsp_types::Location> {
    let url = url(snap, nav.file_id);
    let line_index = snap.file_line_index(nav.file_id)?;
    let range = range(&line_index, nav.full_range);
    let loc = lsp_types::Location::new(url, range);
    Ok(loc)
}

pub(crate) fn location_link(
    snap: &ServerSnapshot,
    src: Option<FileRange>,
    target: NavigationTarget,
) -> Result<lsp_types::LocationLink> {
    let origin_selection_range = match src {
        Some(src) => {
            let line_index = snap.file_line_index(src.file_id)?;
            let range = range(&line_index, src.range);
            Some(range)
        }
        None => None,
    };
    let (target_uri, target_range, target_selection_range) = location_info(snap, target)?;
    let res = lsp_types::LocationLink {
        origin_selection_range,
        target_uri,
        target_range,
        target_selection_range,
    };
    Ok(res)
}

fn location_info(
    snap: &ServerSnapshot,
    target: NavigationTarget,
) -> Result<(lsp_types::Url, lsp_types::Range, lsp_types::Range)> {
    let line_index = snap.file_line_index(target.file_id)?;

    let target_uri = url(snap, target.file_id);
    let target_range = range(&line_index, target.full_range);
    let target_selection_range = target
        .focus_range
        .map(|it| range(&line_index, it))
        .unwrap_or(target_range);
    Ok((target_uri, target_range, target_selection_range))
}

pub(crate) fn goto_definition_response(
    snap: &ServerSnapshot,
    src: Option<FileRange>,
    targets: Vec<NavigationTarget>,
) -> Result<lsp_types::GotoDefinitionResponse> {
    todo!()
}

fn outside_workspace_annotation_id() -> String {
    String::from("OutsideWorkspace")
}

pub(crate) fn snippet_text_document_edit(
    snap: &ServerSnapshot,
    is_snippet: bool,
    file_id: FileID,
    edit: TextEdit,
) -> Result<lsp_ext::SnippetTextDocumentEdit> {
    todo!()
}

pub(crate) fn snippet_text_document_ops(
    snap: &ServerSnapshot,
    file_system_edit: FileSystemEdit,
) -> Cancellable<Vec<lsp_ext::SnippetDocumentChangeOperation>> {
    todo!()
}

pub(crate) fn snippet_workspace_edit(
    snap: &ServerSnapshot,
    source_change: SourceChange,
) -> Result<lsp_ext::SnippetWorkspaceEdit> {
    todo!()
}

pub(crate) fn workspace_edit(
    snap: &ServerSnapshot,
    source_change: SourceChange,
) -> Result<lsp_types::WorkspaceEdit> {
    assert!(!source_change.is_snippet);
    snippet_workspace_edit(snap, source_change).map(|it| it.into())
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

pub(crate) fn call_hierarchy_item(
    snap: &ServerSnapshot,
    target: NavigationTarget,
) -> Result<lsp_types::CallHierarchyItem> {
    let name = target.name.to_string();
    let detail = target.description.clone();
    let kind = target
        .kind
        .map(symbol_kind)
        .unwrap_or(lsp_types::SymbolKind::FUNCTION);
    let (uri, range, selection_range) = location_info(snap, target)?;
    Ok(lsp_types::CallHierarchyItem {
        name,
        kind,
        tags: None,
        detail,
        uri,
        range,
        selection_range,
        data: None,
    })
}

pub(crate) fn code_action_kind(kind: AssistKind) -> lsp_types::CodeActionKind {
    match kind {
        AssistKind::None | AssistKind::Generate => lsp_types::CodeActionKind::EMPTY,
        AssistKind::QuickFix => lsp_types::CodeActionKind::QUICKFIX,
        AssistKind::Refactor => lsp_types::CodeActionKind::REFACTOR,
        AssistKind::RefactorExtract => lsp_types::CodeActionKind::REFACTOR_EXTRACT,
        AssistKind::RefactorInline => lsp_types::CodeActionKind::REFACTOR_INLINE,
        AssistKind::RefactorRewrite => lsp_types::CodeActionKind::REFACTOR_REWRITE,
    }
}

pub(crate) fn code_action(
    snap: &ServerSnapshot,
    assist: Assist,
    resolve_data: Option<(usize, lsp_types::CodeActionParams)>,
) -> Result<lsp_ext::CodeAction> {
    todo!()
}

pub(crate) fn code_lens(
    acc: &mut Vec<lsp_types::CodeLens>,
    snap: &ServerSnapshot,
    annotation: Annotation,
) -> Result<()> {
    todo!()
}

pub(crate) mod command {
    use ide::{FileRange, NavigationTarget};
    use serde_json::to_value;

    use crate::{
        convert::to_lsp_types::{location, location_link},
        lsp_ext,
        server_snapshot::ServerSnapshot,
    };

    pub(crate) fn show_references(
        title: String,
        uri: &lsp_types::Url,
        position: lsp_types::Position,
        locations: Vec<lsp_types::Location>,
    ) -> lsp_types::Command {
        // We cannot use the 'editor.action.showReferences' command directly
        // because that command requires vscode types which we convert in the handler
        // on the client side.

        lsp_types::Command {
            title,
            command: "rust-analyzer.showReferences".into(),
            arguments: Some(vec![
                to_value(uri).unwrap(),
                to_value(position).unwrap(),
                to_value(locations).unwrap(),
            ]),
        }
    }

    pub(crate) fn run_single(runnable: &lsp_ext::Runnable, title: &str) -> lsp_types::Command {
        lsp_types::Command {
            title: title.to_string(),
            command: "rust-analyzer.runSingle".into(),
            arguments: Some(vec![to_value(runnable).unwrap()]),
        }
    }

    pub(crate) fn debug_single(runnable: &lsp_ext::Runnable) -> lsp_types::Command {
        lsp_types::Command {
            title: "Debug".into(),
            command: "rust-analyzer.debugSingle".into(),
            arguments: Some(vec![to_value(runnable).unwrap()]),
        }
    }

    pub(crate) fn goto_location(
        snap: &ServerSnapshot,
        nav: &NavigationTarget,
    ) -> Option<lsp_types::Command> {
        todo!()
    }

    pub(crate) fn trigger_parameter_hints() -> lsp_types::Command {
        lsp_types::Command {
            title: "triggerParameterHints".into(),
            command: "editor.action.triggerParameterHints".into(),
            arguments: None,
        }
    }
}

pub(crate) fn implementation_title(count: usize) -> String {
    if count == 1 {
        "1 implementation".into()
    } else {
        format!("{} implementations", count)
    }
}

pub(crate) fn reference_title(count: usize) -> String {
    if count == 1 {
        "1 reference".into()
    } else {
        format!("{} references", count)
    }
}

pub(crate) fn markup_content(
    markup: Markup,
    kind: ide::HoverDocFormat,
) -> lsp_types::MarkupContent {
    let kind = match kind {
        ide::HoverDocFormat::Markdown => lsp_types::MarkupKind::Markdown,
        ide::HoverDocFormat::PlainText => lsp_types::MarkupKind::PlainText,
    };
    let value = crate::markdown::format_docs(markup.as_str());
    lsp_types::MarkupContent { kind, value }
}

pub(crate) fn rename_error(err: RenameError) -> crate::lsp_error::LspError {
    // This is wrong, but we don't have a better alternative I suppose?
    // https://github.com/microsoft/language-server-protocol/issues/1341
    invalid_params_error(err.to_string())
}
