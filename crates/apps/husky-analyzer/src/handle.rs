mod semantic_tokens;

pub(crate) use semantic_tokens::*;

type AnalyzerDBSnapshot = salsa::Snapshot<AnalyzerDB>;

use crate::lsp_ext::{self, InlayHint, InlayHintsParams, WorkspaceSymbolParams};
use crate::{convert::from_lsp_types, *};
use husky_folding_range::FoldingRangeDb;
use husky_hover::{HoverDb, HoverResult};
use husky_semantic_token::SemanticTokenDb;
use husky_text::{TextPosition, TextRange};
use husky_vfs::VfsDb;
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeLens, CompletionItem, DocumentFormattingParams, FoldingRange, FoldingRangeParams, Location,
    Position, PrepareRenameResponse, RenameParams, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SymbolInformation, WorkspaceEdit,
};
use std::sync::atomic::Ordering;

pub(crate) fn handle_selection_range(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_matching_brace(
    _comptime: AnalyzerDBSnapshot,
    _params: lsp_ext::MatchingBraceParams,
) -> Result<Vec<Position>> {
    msg_once!("todo!");
    Ok(Vec::new())
}

pub(crate) fn handle_on_enter(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<Vec<lsp_ext::SnippetTextEdit>>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_on_type_formatting(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("TODO: handle_on_type_formatting");
    Ok(None)
}

pub(crate) fn handle_document_symbol(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::DocumentSymbolParams,
) -> Result<Option<lsp_types::DocumentSymbolResponse>> {
    msg_once!("todo handle_document_symbol");
    Ok(None)
}

pub(crate) fn handle_workspace_symbol(
    _snapshot: AnalyzerDBSnapshot,
    _params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    msg_once!("todo handle workspace symbol!");
    Ok(None)
}

pub(crate) fn handle_will_rename_files(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::RenameFilesParams,
) -> Result<Option<lsp_types::WorkspaceEdit>> {
    msg_once!("todo handle will rename files!");
    Ok(None)
}

pub(crate) fn handle_goto_definition(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::GotoDefinitionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    msg_once!("todo goto definition!");
    Ok(None)
}

pub(crate) fn handle_goto_declaration(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::request::GotoDeclarationParams,
) -> Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    msg_once!("todo goto declaration!");
    Ok(None)
}

pub(crate) fn handle_goto_type_definition(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::request::GotoTypeDefinitionParams,
) -> Result<Option<lsp_types::request::GotoTypeDefinitionResponse>> {
    msg_once!("todo goto type definition!");
    Ok(None)
}

pub(crate) fn handle_parent_module(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    msg_once!("todo handle parent module!");
    Ok(None)
}

pub(crate) fn handle_completion(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    msg_once!("todo handle completion!");
    Ok(None)
    // let position = FilePosition::from_proto(&*comptime, &params.text_document_position);
    // let completion_trigger_character = params.context.and_then(|ctx| ctx.trigger_character);
    // Ok(snapshot.completion(position, completion_trigger_character))
}

pub(crate) fn handle_completion_resolve(
    _snapshot: AnalyzerDBSnapshot,
    mut _original_completion: CompletionItem,
) -> Result<CompletionItem> {
    msg_once!("todo handle completion resolve");
    Ok(CompletionItem::default())
}

pub(crate) fn handle_folding_range(
    snapshot: AnalyzerDBSnapshot,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    let path = from_lsp_types::path_from_url(&params.text_document.uri)?;
    let module = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    match snapshot.folding_ranges(module) {
        Ok(folding_ranges) => Ok(Some(folding_ranges.to_vec())),
        Err(e) => Err(Box::new(e.clone())),
    }
}

pub(crate) fn handle_decl_help(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::SignatureHelpParams,
) -> Result<Option<lsp_types::SignatureHelp>> {
    msg_once!("todo handle signature help!");
    Ok(None)
}

pub(crate) fn handle_hover(
    snapshot: AnalyzerDBSnapshot,
    params: lsp_ext::HoverParams,
) -> Result<Option<HoverResult>> {
    let path = from_lsp_types::path_from_url(&params.text_document.uri)?;
    let module_path = snapshot.resolve_module_path(snapshot.current_toolchain()?, &path)?;
    let position = match params.position {
        lsp_ext::PositionOrRange::Position(position) => position,
        lsp_ext::PositionOrRange::Range(range) => range.start,
    };
    let position: TextPosition = position.into();
    snapshot
        .hover_result(module_path, position)
        .map_err(|e| e.into())
}

pub(crate) fn handle_prepare_rename(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<PrepareRenameResponse>> {
    msg_once!("todo handle prepare rename!");
    Ok(None)
}

pub(crate) fn handle_rename(
    _snapshot: AnalyzerDBSnapshot,
    _params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    msg_once!("todo handle rename!");
    Ok(None)
}

pub(crate) fn handle_references(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    msg_once!("todo handle references!");
    Ok(None)
}

pub(crate) fn handle_formatting(
    _snapshot: AnalyzerDBSnapshot,
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("todo handle formatting!");
    Ok(None)
}

pub(crate) fn handle_range_formatting(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::DocumentRangeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("todo handle range formatting!");
    Ok(None)
}

pub(crate) fn handle_code_action(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::CodeActionParams,
) -> Result<Option<Vec<lsp_ext::CodeAction>>> {
    msg_once!("todo handle code action!");
    Ok(None)
}

pub(crate) fn handle_code_action_resolve(
    _snapshot: AnalyzerDBSnapshot,
    mut _code_action: lsp_ext::CodeAction,
) -> Result<lsp_ext::CodeAction> {
    msg_once!("todo handle code action resolve!");
    Ok(lsp_ext::CodeAction::default())
}

pub(crate) fn handle_code_lens(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    Ok(None)
}

pub(crate) fn handle_code_lens_resolve(
    _snapshot: AnalyzerDBSnapshot,
    _code_lens: CodeLens,
) -> Result<CodeLens> {
    msg_once!("todo handle code lens resolve!");
    todo!()
}

pub(crate) fn handle_document_highlight(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::DocumentHighlightParams,
) -> Result<Option<Vec<lsp_types::DocumentHighlight>>> {
    msg_once!("todo handle document highlight!");
    Ok(None)
}

pub(crate) fn handle_ssr(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_ext::SsrParams,
) -> Result<lsp_types::WorkspaceEdit> {
    msg_once!("todo handle ssr");
    todo!()
}

pub(crate) fn handle_inlay_hints(
    _snapshot: AnalyzerDBSnapshot,
    _params: InlayHintsParams,
) -> Result<Vec<InlayHint>> {
    msg_once!("todo handle inlay hints");
    todo!()
}

pub(crate) fn handle_call_hierarchy_prepare(
    _snapshot: AnalyzerDBSnapshot,
    _params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    msg_once!("todo handle call hierarchy prepare");
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_incoming(
    _snapshot: AnalyzerDBSnapshot,
    _params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    msg_once!("todo handle call hierarchy incoming");
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_outgoing(
    _snapshot: AnalyzerDBSnapshot,
    _params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    msg_once!("todo handle call hierarchy outgoing");
    Ok(None)
}

pub(crate) fn handle_open_docs(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::Url>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_move_item(
    _snapshot: AnalyzerDBSnapshot,
    _params: lsp_ext::MoveItemParams,
) -> Result<Vec<lsp_ext::SnippetTextEdit>> {
    msg_once!("todo handle move item");
    todo!()
}
