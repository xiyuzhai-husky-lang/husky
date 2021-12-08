mod handlers;
use common::*;

use crate::{
    convert::from_lsp_types,
    lsp_ext,
    run_server::dispatch::{NotificationDispatcher, RequestDispatcher},
    server::{Server, ServerControlSignal},
};

pub(crate) fn handle_lsp_msg(
    server: &mut Server,
    msg: lsp_server::Message,
    loop_start: std::time::Instant,
) -> Result<ServerControlSignal> {
    match msg {
        lsp_server::Message::Request(req) => handle_lsp_request(server, loop_start, req),
        lsp_server::Message::Notification(notif) => handle_lsp_notification(server, notif),
        lsp_server::Message::Response(resp) => handle_lsp_request_complete(server, resp),
    }
}

fn handle_lsp_request(
    server: &mut Server,
    instant_when_received_request: std::time::Instant,
    req: lsp_server::Request,
) -> Result<ServerControlSignal> {
    // ep!(req);
    server.comm.req_queue.incoming.register(
        req.id.clone(),
        (req.method.clone(), instant_when_received_request),
    );
    let mut dispatcher = RequestDispatcher {
        req: Some(req),
        server,
        control_signal: ServerControlSignal::Normal,
    };
    dispatcher
        .on_sync::<lsp_ext::JoinLines>(handlers::handle_join_lines)?
        .on_sync::<lsp_ext::OnEnter>(handlers::handle_on_enter)?
        .on_sync::<lsp_types::request::SelectionRangeRequest>(handlers::handle_selection_range)?
        .on_sync::<lsp_ext::MatchingBrace>(handlers::handle_matching_brace)?
        .on::<lsp_ext::AnalyzerStatus>(handlers::handle_analyzer_status)
        .on::<lsp_ext::SyntaxTree>(handlers::handle_syntax_tree)
        .on::<lsp_ext::ViewHir>(handlers::handle_view_hir)
        .on::<lsp_ext::ViewItemTree>(handlers::handle_view_item_tree)
        .on::<lsp_ext::ParentModule>(handlers::handle_parent_module)
        .on::<lsp_ext::Runnables>(handlers::handle_runnables)
        .on::<lsp_ext::RelatedTests>(handlers::handle_related_tests)
        .on::<lsp_ext::InlayHints>(handlers::handle_inlay_hints)
        .on::<lsp_ext::CodeActionRequest>(handlers::handle_code_action)
        .on::<lsp_ext::CodeActionResolveRequest>(handlers::handle_code_action_resolve)
        .on::<lsp_ext::HoverRequest>(handlers::handle_hover)
        .on::<lsp_ext::ExternalDocs>(handlers::handle_open_docs)
        .on::<lsp_ext::OpenCargoToml>(handlers::handle_open_cargo_toml)
        .on::<lsp_ext::MoveItem>(handlers::handle_move_item)
        .on::<lsp_ext::WorkspaceSymbol>(handlers::handle_workspace_symbol)
        .on::<lsp_types::request::OnTypeFormatting>(handlers::handle_on_type_formatting)
        .on::<lsp_types::request::DocumentSymbolRequest>(handlers::handle_document_symbol)
        .on::<lsp_types::request::GotoDefinition>(handlers::handle_goto_definition)
        .on::<lsp_types::request::GotoDeclaration>(handlers::handle_goto_declaration)
        .on::<lsp_types::request::GotoImplementation>(handlers::handle_goto_implementation)
        .on::<lsp_types::request::GotoTypeDefinition>(handlers::handle_goto_type_definition)
        .on::<lsp_types::request::Completion>(handlers::handle_completion)
        .on::<lsp_types::request::ResolveCompletionItem>(handlers::handle_completion_resolve)
        .on::<lsp_types::request::CodeLensRequest>(handlers::handle_code_lens)
        .on::<lsp_types::request::CodeLensResolve>(handlers::handle_code_lens_resolve)
        .on::<lsp_types::request::FoldingRangeRequest>(handlers::handle_folding_range)
        .on::<lsp_types::request::SignatureHelpRequest>(handlers::handle_signature_help)
        .on::<lsp_types::request::PrepareRenameRequest>(handlers::handle_prepare_rename)
        .on::<lsp_types::request::Rename>(handlers::handle_rename)
        .on::<lsp_types::request::References>(handlers::handle_references)
        .on::<lsp_types::request::Formatting>(handlers::handle_formatting)
        .on::<lsp_types::request::RangeFormatting>(handlers::handle_range_formatting)
        .on::<lsp_types::request::DocumentHighlightRequest>(handlers::handle_document_highlight)
        .on::<lsp_types::request::CallHierarchyPrepare>(handlers::handle_call_hierarchy_prepare)
        .on::<lsp_types::request::CallHierarchyIncomingCalls>(
            handlers::handle_call_hierarchy_incoming,
        )
        .on::<lsp_types::request::CallHierarchyOutgoingCalls>(
            handlers::handle_call_hierarchy_outgoing,
        )
        .on::<lsp_types::request::SemanticTokensFullRequest>(handlers::handle_semantic_tokens_full)
        .on::<lsp_types::request::SemanticTokensFullDeltaRequest>(
            handlers::handle_semantic_tokens_full_delta,
        )
        .on::<lsp_types::request::SemanticTokensRangeRequest>(
            handlers::handle_semantic_tokens_range,
        )
        .on::<lsp_types::request::WillRenameFiles>(handlers::handle_will_rename_files)
        .on::<lsp_ext::Ssr>(handlers::handle_ssr)
        .on_control::<lsp_types::request::Shutdown>(|_, _| ServerControlSignal::Shutdown)?
        .finish();
    Ok(dispatcher.control_signal)
}

fn handle_lsp_notification(
    server: &mut Server,
    notif: lsp_server::Notification,
) -> Result<ServerControlSignal> {
    NotificationDispatcher {
        notif: Some(notif),
        server,
    }
    .on::<lsp_types::notification::Cancel>(|_this, params| {
        ep!(params);
        let _id: lsp_server::RequestId = match params.id {
            lsp_types::NumberOrString::Number(id) => id.into(),
            lsp_types::NumberOrString::String(id) => id.into(),
        };
        eprintln!("TODO: on::<lsp_types::notification::Cancel>");
        // this.cancel(id);
        // todo!();
        Ok(())
    })?
    .on::<lsp_types::notification::WorkDoneProgressCancel>(|_this, _params| {
        // Just ignore this. It is OK to continue sending progress
        // notifications for this token, as the client can't know when
        // we accepted notification.
        Ok(())
    })?
    .on::<lsp_types::notification::DidOpenTextDocument>(handle_did_open_text_document)?
    .on::<lsp_types::notification::DidChangeTextDocument>(|_this, _params| {
        todo!();
    })?
    .on::<lsp_types::notification::DidCloseTextDocument>(|_this, _params| {
        todo!();
    })?
    .on::<lsp_types::notification::DidSaveTextDocument>(|_this, _params| {
        todo!();
    })?
    .on::<lsp_types::notification::DidChangeConfiguration>(|_this, _params| {
        todo!();
    })?
    .on::<lsp_types::notification::DidChangeWatchedFiles>(|_this, _params| {
        todo!();
    })?
    .finish();
    return Ok(ServerControlSignal::Normal);

    fn handle_did_open_text_document(
        this: &mut Server,
        params: lsp_types::DidOpenTextDocumentParams,
    ) -> Result<()> {
        use vfs::VirtualFileSystem;

        if let Ok(path) = from_lsp_types::vfs_path(&params.text_document.uri) {
            this.db.set_live(path, params.text_document.text);
        }
        Ok(())
    }
}

fn handle_lsp_request_complete(
    server: &mut Server,
    response: lsp_server::Response,
) -> Result<ServerControlSignal> {
    let handler = server.comm.req_queue.outgoing.complete(response.id.clone());
    handler(server, response);
    Ok(ServerControlSignal::Normal)
}
