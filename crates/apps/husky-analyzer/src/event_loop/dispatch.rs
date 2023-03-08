mod dispatcher;

use crate::{
    convert::from_lsp_types,
    event_loop::dispatch::dispatcher::{NotificationDispatcher, RequestDispatcher},
    lsp_ext,
    server::{Server, TaskSet},
    *,
};

use handle::*;

use husky_vfs::{apply_live_file_changes, set_live_file};

pub(crate) fn dispatch_lsp_msg(
    server: &mut Server,
    msg: lsp_server::Message,
    loop_start: std::time::Instant,
) -> Result<TaskSet> {
    match msg {
        lsp_server::Message::Request(req) => dispatch_lsp_request(server, loop_start, req),
        lsp_server::Message::Notification(notif) => handle_lsp_notification(server, notif),
        lsp_server::Message::Response(resp) => handle_lsp_response(server, resp),
    }
}

fn dispatch_lsp_request(
    server: &mut Server,
    instant_when_received_request: std::time::Instant,
    req: lsp_server::Request,
) -> Result<TaskSet> {
    server.client_comm.req_queue.incoming.register(
        req.id.clone(),
        (req.method.clone(), instant_when_received_request),
    );
    let mut dispatcher = RequestDispatcher {
        req: Some(req),
        server,
        control_signal: TaskSet::Nothing,
    };
    dispatcher
        .on_sync::<lsp_ext::OnEnter>(handle_on_enter)?
        .on_sync::<lsp_types::request::SelectionRangeRequest>(handle_selection_range)?
        .on_sync::<lsp_ext::MatchingBrace>(handle_matching_brace)?
        .on::<lsp_ext::ParentModule>(handle_parent_module)
        .on::<lsp_ext::InlayHints>(handle_inlay_hints)
        .on::<lsp_ext::CodeActionRequest>(handle_code_action)
        .on::<lsp_ext::CodeActionResolveRequest>(handle_code_action_resolve)
        .on::<lsp_ext::HoverRequest>(handle_hover)
        .on::<lsp_ext::ExternalDocs>(handle_open_docs)
        .on::<lsp_ext::MoveItem>(handle_move_item)
        .on::<lsp_ext::WorkspaceSymbol>(handle_workspace_symbol)
        .on::<lsp_types::request::OnTypeFormatting>(handle_on_type_formatting)
        .on::<lsp_types::request::DocumentSymbolRequest>(handle_document_symbol)
        .on::<lsp_types::request::GotoDefinition>(handle_goto_definition)
        .on::<lsp_types::request::GotoDeclaration>(handle_goto_declaration)
        .on::<lsp_types::request::GotoTypeDefinition>(handle_goto_type_definition)
        .on::<lsp_types::request::Completion>(handle_completion)
        .on::<lsp_types::request::ResolveCompletionItem>(handle_completion_resolve)
        .on::<lsp_types::request::CodeLensRequest>(handle_code_lens)
        .on::<lsp_types::request::CodeLensResolve>(handle_code_lens_resolve)
        .on::<lsp_types::request::FoldingRangeRequest>(handle_folding_range)
        .on::<lsp_types::request::SignatureHelpRequest>(handle_decl_help)
        .on::<lsp_types::request::PrepareRenameRequest>(handle_prepare_rename)
        .on::<lsp_types::request::Rename>(handle_rename)
        .on::<lsp_types::request::References>(handle_references)
        .on::<lsp_types::request::Formatting>(handle_formatting)
        .on::<lsp_types::request::RangeFormatting>(handle_range_formatting)
        .on::<lsp_types::request::DocumentHighlightRequest>(handle_document_highlight)
        .on::<lsp_types::request::CallHierarchyPrepare>(handle_call_hierarchy_prepare)
        .on::<lsp_types::request::CallHierarchyIncomingCalls>(handle_call_hierarchy_incoming)
        .on::<lsp_types::request::CallHierarchyOutgoingCalls>(handle_call_hierarchy_outgoing)
        .on::<lsp_types::request::SemanticTokensFullRequest>(handle_semantic_tokens_full)
        .on::<lsp_types::request::SemanticTokensFullDeltaRequest>(handle_semantic_tokens_full_delta)
        .on::<lsp_types::request::SemanticTokensRangeRequest>(handle_semantic_tokens_range)
        .on::<lsp_types::request::WillRenameFiles>(handle_will_rename_files)
        .on::<lsp_ext::Ssr>(handle_ssr)
        .on_control::<lsp_types::request::Shutdown>(|_, _| TaskSet::Shutdown)?
        .finish();
    Ok(dispatcher.control_signal)
}

fn handle_lsp_notification(
    server: &mut Server,
    notif: lsp_server::Notification,
) -> Result<TaskSet> {
    let mut dispatcher = NotificationDispatcher {
        notif: Some(notif),
        server,
        task: TaskSet::Nothing,
    };
    dispatcher
        .on_sync::<lsp_types::notification::Cancel>(|_server, params| {
            let _id: lsp_server::RequestId = match params.id {
                lsp_types::NumberOrString::Number(id) => id.into(),
                lsp_types::NumberOrString::String(id) => id.into(),
            };
            msg_once!("TODO: on::<lsp_types::notification::Cancel>");
            Ok(TaskSet::Nothing)
        })?
        .on_sync::<lsp_types::notification::WorkDoneProgressCancel>(|_server, _params| {
            // Just ignore this. It is OK to continue sending progress
            // notifications for this token, as the client can't know when
            // we accepted notification.
            Ok(TaskSet::Nothing)
        })?
        .on_sync::<lsp_types::notification::DidOpenTextDocument>(|server, params| {
            if let Ok(path) = from_lsp_types::path_from_url(&params.text_document.uri) {
                eprintln!("set live file for path {:?}", path);
                match set_live_file(&mut server.db, &path, params.text_document.text) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!(
                            "error when setting live file at {}: {e}",
                            params.text_document.uri
                        )
                    }
                }
            }
            Ok(TaskSet::SendUpdates)
        })?
        .on_sync::<lsp_types::notification::DidChangeTextDocument>(|server, params| {
            if let Ok(path) = from_lsp_types::path_from_url(&params.text_document.uri) {
                eprintln!("apply live file changes for path {:?}", path);
                let changes = params
                    .content_changes
                    .into_iter()
                    .map(|change| change.into())
                    .collect();
                apply_live_file_changes(&mut server.db, &path, changes)?;
            }
            Ok(TaskSet::SendUpdates)
        })?
        .on_sync::<lsp_types::notification::DidCloseTextDocument>(|_server, _params| {
            eprintln!("todo: lsp_types::notification::DidCloseTextDocument");
            Ok(TaskSet::SendUpdates)
        })?
        .on_sync::<lsp_types::notification::DidSaveTextDocument>(|_server, params| {
            if let Ok(_path) = from_lsp_types::path_from_url(&params.text_document.uri) {
                if let Some(_text) = params.text {
                    todo!()
                }
            }
            Ok(TaskSet::Nothing)
        })?
        .on_sync::<lsp_types::notification::DidChangeConfiguration>(|_server, _params| {
            todo!();
        })?
        .on_sync::<lsp_types::notification::DidChangeWatchedFiles>(|_server, _params| {
            todo!();
        })?
        .finish();
    return Ok(dispatcher.task);
}

fn handle_lsp_response(server: &mut Server, response: lsp_server::Response) -> Result<TaskSet> {
    let handler = server
        .client_comm
        .req_queue
        .outgoing
        .complete(response.id.clone());
    handler(server, response);
    Ok(TaskSet::Nothing)
}
