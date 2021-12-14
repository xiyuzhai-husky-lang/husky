//! This module is responsible for implementing handlers for Language Server
//! Protocol. The majority of requests are fulfilled by calling into the
//! `ide` crate.

use common::*;
type HuskyLangDatabaseSnapshot = salsa::Snapshot<husky_lang_db::HuskyLangDatabase>;

use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeLens, CompletionItem, DocumentFormattingParams, FoldingRange, FoldingRangeParams, Location,
    Position, PrepareRenameResponse, RenameParams, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SymbolInformation, WorkspaceEdit,
};

use crate::lsp_ext::{self, InlayHint, InlayHintsParams, WorkspaceSymbolParams};

pub(crate) fn handle_selection_range(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_matching_brace(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_ext::MatchingBraceParams,
) -> Result<Vec<Position>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(Vec::new())
}

pub(crate) fn handle_on_enter(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<Vec<lsp_ext::SnippetTextEdit>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_on_type_formatting(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    eprintln!("TODO: handle_on_type_formatting");
    Ok(None)
}

pub(crate) fn handle_document_symbol(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::DocumentSymbolParams,
) -> Result<Option<lsp_types::DocumentSymbolResponse>> {
    eprintln!("TODO: handle_document_symbol");
    Ok(None)
}

pub(crate) fn handle_workspace_symbol(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
    // let _p = profile::span("handle_workspace_symbol");

    // let (all_symbols, libs) = decide_search_scope_and_kind(&params, &snap);

    // let query = {
    //     let query: String = params
    //         .query
    //         .chars()
    //         .filter(|&c| c != '#' && c != '*')
    //         .collect();
    //     let mut q = Query::new(query);
    //     if !all_symbols {
    //         q.only_types();
    //     }
    //     if libs {
    //         q.libs();
    //     }
    //     q.limit(128);
    //     q
    // };
    // let mut res = exec_query(&snap, query)?;
    // if res.is_empty() && !all_symbols {
    //     let mut query = Query::new(params.query);
    //     query.limit(128);
    //     res = exec_query(&snap, query)?;
    // }

    // return Ok(Some(res));

    // fn decide_search_scope_and_kind(
    //     _params: &WorkspaceSymbolParams,
    //     _snapshot: &HuskyLangDatabaseSnapshot,
    // ) -> (bool, bool) {
    //     eprintln!("{}:{} todo!", file!(), line!()); Ok(None)
    // }

    // fn exec_query(snap: &HuskyLangDatabaseSnapshot, query: Query) -> Result<Vec<SymbolInformation>> {
    //     eprintln!("{}:{} todo!", file!(), line!()); Ok(None)
    // }
}

pub(crate) fn handle_will_rename_files(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::RenameFilesParams,
) -> Result<Option<lsp_types::WorkspaceEdit>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_goto_definition(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::GotoDefinitionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_goto_declaration(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::request::GotoDeclarationParams,
) -> Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_goto_type_definition(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::request::GotoTypeDefinitionParams,
) -> Result<Option<lsp_types::request::GotoTypeDefinitionResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_parent_module(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_completion(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_completion_resolve(
    _snapshot: HuskyLangDatabaseSnapshot,
    mut _original_completion: CompletionItem,
) -> Result<CompletionItem> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(CompletionItem::default())
}

pub(crate) fn handle_folding_range(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    eprintln!("TODO: handle_folding_range");
    Ok(None)
    // let _p = profile::span("handle_folding_range");
    // let file_id = from_lsp_types::to_file_id(&snapshot, &params.text_document.uri)?;
    // let folds = snapshot.db.folding_ranges(file_id)?;
    // let text = snapshot.db.file_text(file_id)?;
    // let line_index = snapshot.db.get_file_line_collection(file_id)?;
    // let line_folding_only = snapshot.config.line_folding_only();
    // let res = folds
    //     .into_iter()
    //     .map(|it| to_lsp_types::folding_range(&*text, &line_index, line_folding_only, it))
    //     .collect();
    // Ok(Some(res))
}

pub(crate) fn handle_signature_help(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::SignatureHelpParams,
) -> Result<Option<lsp_types::SignatureHelp>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_hover(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_ext::HoverParams,
) -> Result<Option<lsp_ext::Hover>> {
    Ok(None)
}

pub(crate) fn handle_prepare_rename(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<PrepareRenameResponse>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_rename(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_references(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_formatting(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_range_formatting(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::DocumentRangeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_code_action(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::CodeActionParams,
) -> Result<Option<Vec<lsp_ext::CodeAction>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_code_action_resolve(
    _snapshot: HuskyLangDatabaseSnapshot,
    mut _code_action: lsp_ext::CodeAction,
) -> Result<lsp_ext::CodeAction> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(lsp_ext::CodeAction::default())
}

pub(crate) fn handle_code_lens(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    Ok(None)
}

pub(crate) fn handle_code_lens_resolve(
    _snapshot: HuskyLangDatabaseSnapshot,
    _code_lens: CodeLens,
) -> Result<CodeLens> {
    eprintln!("{}:{} todo!", file!(), line!());
    todo!()
}

pub(crate) fn handle_document_highlight(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::DocumentHighlightParams,
) -> Result<Option<Vec<lsp_types::DocumentHighlight>>> {
    eprintln!("{}:{} todo handle_document_highlight!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_ssr(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_ext::SsrParams,
) -> Result<lsp_types::WorkspaceEdit> {
    eprintln!("{}:{} todo!", file!(), line!());
    todo!()
}

pub(crate) fn handle_inlay_hints(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: InlayHintsParams,
) -> Result<Vec<InlayHint>> {
    eprintln!("{}:{} todo!", file!(), line!());
    todo!()
}

pub(crate) fn handle_call_hierarchy_prepare(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_incoming(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_outgoing(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_semantic_tokens_full(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    eprintln!("handle_semantic_tokens_full");
    Ok(None)
    // let _p = profile::span("handle_semantic_tokens_full");

    // let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    // let text = snap.analysis.file_text(file_id)?;
    // let line_index = snap.file_line_collection(file_id)?;

    // let highlights = snap.analysis.highlight(file_id)?;
    // let highlight_strings = snap.config.highlighting_strings();
    // let semantic_tokens =
    //     to_lsp_types::to_semantic_tokens(&text, &line_index, highlights, highlight_strings);

    // use salsa database

    // // Unconditionally cache the tokens
    // snap.semantic_tokens_cache
    //     .lock()
    //     .insert(params.text_document.uri, semantic_tokens.clone());

    // Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_semantic_tokens_range(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    eprintln!("TODO: semantic tokens range");
    Ok(None)
    // eprintln!("{}:{} todo!", file!(), line!()); Ok(None)
    // let _p = profile::span("handle_semantic_tokens_range");

    // let frange = from_lsp_types::file_range(&snapshot, params.text_document, params.range)?;
    // let text = snapshot.analysis.file_text(frange.file_id)?;
    // let line_index = snapshot.file_line_collection(frange.file_id)?;

    // let highlights = snapshot.analysis.highlight_range(frange)?;
    // let highlight_strings = snapshot.config.highlighting_strings();
    // let semantic_tokens =
    //     to_lsp_types::to_semantic_tokens(&text, &line_index, highlights, highlight_strings);
    // Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_open_docs(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::Url>> {
    eprintln!("{}:{} todo!", file!(), line!());
    Ok(None)
}

pub(crate) fn handle_move_item(
    _snapshot: HuskyLangDatabaseSnapshot,
    _params: lsp_ext::MoveItemParams,
) -> Result<Vec<lsp_ext::SnippetTextEdit>> {
    eprintln!("{}:{} todo!", file!(), line!());
    todo!()
}
