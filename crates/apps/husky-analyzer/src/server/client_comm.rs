use std::time::Instant;

use crossbeam_channel::Sender;

use dashmap::{mapref::entry::Entry, DashMap};
use husky_diagnostics::{DiagnosticSheet, DiagnosticsDb};
use husky_vfs::{ModulePath, VfsDb};
use lsp_types::notification::Notification;

use crate::{convert::to_lsp_types::url_from_abs_path, db::AnalyzerDB};

use super::Server;

pub(crate) struct ClientCommunicator {
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) req_queue: ReqQueue,
    diagnostic_sheets_sent: DashMap<ModulePath, DiagnosticSheet>,
}

pub(crate) type ReqHandler = fn(&mut Server, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

impl ClientCommunicator {
    pub(super) fn new(sender: Sender<lsp_server::Message>) -> ClientCommunicator {
        ClientCommunicator {
            sender,
            req_queue: Default::default(),
            diagnostic_sheets_sent: Default::default(),
        }
    }
    fn send(&self, message: lsp_server::Message) {
        self.sender.send(message).unwrap()
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        params: N::Params,
    ) {
        let notif = lsp_server::Notification::new(N::METHOD.to_string(), params);
        self.send(notif.into());
    }

    pub(crate) fn send_diagnostics(&self, db: &AnalyzerDB, module_path: ModulePath) {
        let diagnostic_sheet = db.diagnostic_sheet(module_path);
        let send_flag = match self.diagnostic_sheets_sent.entry(module_path) {
            Entry::Occupied(mut entry) => match *entry.get() == diagnostic_sheet {
                true => false,
                false => {
                    entry.insert(diagnostic_sheet);
                    true
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(diagnostic_sheet);
                true
            }
        };
        if send_flag {
            let Ok(module_diff_path) = db.module_diff_path(module_path) else { todo!() };
            let url = url_from_abs_path(module_diff_path.data(db));
            self.send_diagnostics_aux(
                url,
                diagnostic_sheet
                    .diagnostics(db)
                    .iter()
                    .map(|diagnostic| diagnostic.into())
                    .collect(),
                None,
            )
        }
    }

    fn send_diagnostics_aux(
        &self,
        url: lsp_types::Url,
        diagnostics: Vec<lsp_types::Diagnostic>,
        version: Option<i32>,
    ) {
        let notif = lsp_server::Notification::new(
            lsp_types::notification::PublishDiagnostics::METHOD.to_string(),
            lsp_types::PublishDiagnosticsParams {
                uri: url,
                diagnostics,
                version,
            },
        );
        self.send(notif.into());
    }

    pub(crate) fn _send_request<R: lsp_types::request::Request>(
        &mut self,
        params: R::Params,
        handler: ReqHandler,
    ) {
        let request = self
            .req_queue
            .outgoing
            .register(R::METHOD.to_string(), params, handler);
        self.send(request.into());
    }

    pub(crate) fn respond(&mut self, response: lsp_server::Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(response.id.clone()) {
            if let Some(err) = &response.error {
                if err.message.starts_with("server panicked") {
                    self.poke_husky_developer(format!("{}, check the log", err.message))
                }
            }

            let duration = start.elapsed();
            tracing::info!(
                "handled {} - ({}) in {:0.2?}",
                method,
                response.id,
                duration
            );
            self.send(response.into());
        }
    }

    pub(crate) fn show_message(&mut self, typ: lsp_types::MessageType, message: String) {
        let message = message;
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams { typ, message },
        )
    }

    pub(crate) fn poke_husky_developer(&mut self, message: String) {
        self.show_message(lsp_types::MessageType::ERROR, message)
    }
}
