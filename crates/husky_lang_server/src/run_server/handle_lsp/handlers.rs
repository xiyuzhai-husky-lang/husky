//! This module is responsible for implementing handlers for Language Server
//! Protocol. The majority of requests are fulfilled by calling into the
//! `ide` crate.

use common::*;

use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeLens, CompletionItem, DocumentFormattingParams, FoldingRange, FoldingRangeParams, Location,
    Position, PrepareRenameResponse, RenameParams, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SymbolInformation, WorkspaceEdit,
};
use stdx::format_to;

use crate::{
    lsp_ext::{self, InlayHint, InlayHintsParams, WorkspaceSymbolParams},
    server::Server,
    server_snapshot::ServerSnapshot,
};

pub(crate) fn handle_analyzer_status(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::AnalyzerStatusParams,
) -> Result<String> {
    todo!()
}

pub(crate) fn handle_memory_usage(state: &mut Server, _: ()) -> Result<String> {
    let _p = profile::span("handle_memory_usage");
    let mut mem = state.db.per_query_memory_usage();
    mem.push(("Remaining".into(), profile::memory_usage().allocated));

    let mut out = String::new();
    for (name, bytes) in mem {
        format_to!(out, "{:>8} {}\n", bytes, name);
    }
    Ok(out)
}

pub(crate) fn handle_syntax_tree(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::SyntaxTreeParams,
) -> Result<String> {
    todo!()
}

pub(crate) fn handle_view_hir(
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<String> {
    todo!()
}

pub(crate) fn handle_view_item_tree(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::ViewItemTreeParams,
) -> Result<String> {
    todo!()
}

pub(crate) fn handle_selection_range(
    _snapshot: ServerSnapshot,
    _params: lsp_types::SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    todo!()
}

pub(crate) fn handle_matching_brace(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::MatchingBraceParams,
) -> Result<Vec<Position>> {
    todo!()
}

pub(crate) fn handle_join_lines(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::JoinLinesParams,
) -> Result<Vec<lsp_types::TextEdit>> {
    todo!()
}

pub(crate) fn handle_on_enter(
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<Vec<lsp_ext::SnippetTextEdit>>> {
    todo!()
}

pub(crate) fn handle_on_type_formatting(
    _snapshot: ServerSnapshot,
    _params: lsp_types::DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    todo!()
}

pub(crate) fn handle_document_symbol(
    _snapshot: ServerSnapshot,
    _params: lsp_types::DocumentSymbolParams,
) -> Result<Option<lsp_types::DocumentSymbolResponse>> {
    todo!()
}

pub(crate) fn handle_workspace_symbol(
    _snapshot: ServerSnapshot,
    _params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    todo!()
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
    //     _snapshot: &ServerSnapshot,
    // ) -> (bool, bool) {
    //     todo!()
    // }

    // fn exec_query(snap: &ServerSnapshot, query: Query) -> Result<Vec<SymbolInformation>> {
    //     todo!()
    // }
}

pub(crate) fn handle_will_rename_files(
    _snapshot: ServerSnapshot,
    _params: lsp_types::RenameFilesParams,
) -> Result<Option<lsp_types::WorkspaceEdit>> {
    todo!()
}

pub(crate) fn handle_goto_definition(
    _snapshot: ServerSnapshot,
    _params: lsp_types::GotoDefinitionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_goto_declaration(
    _snapshot: ServerSnapshot,
    _params: lsp_types::request::GotoDeclarationParams,
) -> Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    todo!()
}

pub(crate) fn handle_goto_implementation(
    _snapshot: ServerSnapshot,
    _params: lsp_types::request::GotoImplementationParams,
) -> Result<Option<lsp_types::request::GotoImplementationResponse>> {
    todo!()
}

pub(crate) fn handle_goto_type_definition(
    _snapshot: ServerSnapshot,
    _params: lsp_types::request::GotoTypeDefinitionParams,
) -> Result<Option<lsp_types::request::GotoTypeDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_parent_module(
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_runnables(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::RunnablesParams,
) -> Result<Vec<lsp_ext::Runnable>> {
    todo!()
}

pub(crate) fn handle_related_tests(
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Vec<lsp_ext::TestInfo>> {
    todo!()
}

pub(crate) fn handle_completion(
    _snapshot: ServerSnapshot,
    _params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    todo!()
}

pub(crate) fn handle_completion_resolve(
    _snapshot: ServerSnapshot,
    mut _original_completion: CompletionItem,
) -> Result<CompletionItem> {
    todo!()
}

pub(crate) fn handle_folding_range(
    snapshot: ServerSnapshot,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    todo!()
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
    _snapshot: ServerSnapshot,
    _params: lsp_types::SignatureHelpParams,
) -> Result<Option<lsp_types::SignatureHelp>> {
    todo!()
}

pub(crate) fn handle_hover(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::HoverParams,
) -> Result<Option<lsp_ext::Hover>> {
    Ok(None)
}

pub(crate) fn handle_prepare_rename(
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<PrepareRenameResponse>> {
    todo!()
}

pub(crate) fn handle_rename(
    _snapshot: ServerSnapshot,
    _params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    todo!()
}

pub(crate) fn handle_references(
    _snapshot: ServerSnapshot,
    _params: lsp_types::ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    todo!()
}

pub(crate) fn handle_formatting(
    _snapshot: ServerSnapshot,
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    todo!()
}

pub(crate) fn handle_range_formatting(
    _snapshot: ServerSnapshot,
    _params: lsp_types::DocumentRangeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    todo!()
}

pub(crate) fn handle_code_action(
    _snapshot: ServerSnapshot,
    _params: lsp_types::CodeActionParams,
) -> Result<Option<Vec<lsp_ext::CodeAction>>> {
    todo!()
}

pub(crate) fn handle_code_action_resolve(
    _snapshot: ServerSnapshot,
    mut _code_action: lsp_ext::CodeAction,
) -> Result<lsp_ext::CodeAction> {
    todo!()
}

pub(crate) fn handle_code_lens(
    _snapshot: ServerSnapshot,
    _params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    Ok(None)
}

pub(crate) fn handle_code_lens_resolve(
    _snapshot: ServerSnapshot,
    _code_lens: CodeLens,
) -> Result<CodeLens> {
    todo!()
}

pub(crate) fn handle_document_highlight(
    _snapshot: ServerSnapshot,
    _params: lsp_types::DocumentHighlightParams,
) -> Result<Option<Vec<lsp_types::DocumentHighlight>>> {
    todo!()
}

pub(crate) fn handle_ssr(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::SsrParams,
) -> Result<lsp_types::WorkspaceEdit> {
    todo!()
}

pub(crate) fn handle_inlay_hints(
    _snapshot: ServerSnapshot,
    _params: InlayHintsParams,
) -> Result<Vec<InlayHint>> {
    todo!()
}

pub(crate) fn handle_call_hierarchy_prepare(
    _snapshot: ServerSnapshot,
    _params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    todo!()
}

pub(crate) fn handle_call_hierarchy_incoming(
    _snapshot: ServerSnapshot,
    _params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    todo!()
}

pub(crate) fn handle_call_hierarchy_outgoing(
    _snapshot: ServerSnapshot,
    _params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    todo!()
}

pub(crate) fn handle_semantic_tokens_full(
    _snapshot: ServerSnapshot,
    _params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    // let _p = profile::span("handle_semantic_tokens_full");

    // let file_id = from_lsp_types::to_file_id(&snap, &params.text_document.uri)?;
    // let text = snap.analysis.file_text(file_id)?;
    // let line_index = snap.file_line_collection(file_id)?;

    // let highlights = snap.analysis.highlight(file_id)?;
    // let highlight_strings = snap.config.highlighting_strings();
    // let semantic_tokens =
    //     to_lsp_types::to_semantic_tokens(&text, &line_index, highlights, highlight_strings);

    // use salsa database
    todo!();

    // // Unconditionally cache the tokens
    // snap.semantic_tokens_cache
    //     .lock()
    //     .insert(params.text_document.uri, semantic_tokens.clone());

    // Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    _snapshot: ServerSnapshot,
    _params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    todo!()
}

pub(crate) fn handle_semantic_tokens_range(
    _snapshot: ServerSnapshot,
    _params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    todo!()
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
    _snapshot: ServerSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::Url>> {
    todo!()
}

pub(crate) fn handle_open_cargo_toml(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::OpenCargoTomlParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    todo!()
}

pub(crate) fn handle_move_item(
    _snapshot: ServerSnapshot,
    _params: lsp_ext::MoveItemParams,
) -> Result<Vec<lsp_ext::SnippetTextEdit>> {
    todo!()
}
