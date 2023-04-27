use std::time::Instant;

use crossbeam_channel::Sender;

use dashmap::{mapref::entry::Entry, DashMap};
use husky_diagnostics::DiagnosticsDb;
use husky_vfs::{ModulePath, VfsDb};
use lsp_types::notification::Notification;
use salsa::DebugWithDb;

use crate::{convert::to_lsp_types::url_from_diff_path, db::AnalyzerDB};

use super::Server;

pub(crate) struct ClientCommunicator {
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) req_queue: ReqQueue,
    diagnostics_sent: DashMap<ModulePath, Vec<lsp_types::Diagnostic>>,
}

pub(crate) type ReqHandler = fn(&mut Server, lsp_server::Response);
pub(crate) type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

impl ClientCommunicator {
    pub(super) fn new(sender: Sender<lsp_server::Message>) -> ClientCommunicator {
        ClientCommunicator {
            sender,
            req_queue: Default::default(),
            diagnostics_sent: Default::default(),
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
        const DEBUG_SEND_DIAGNOSTICS: bool = false;
        if DEBUG_SEND_DIAGNOSTICS {
            eprintln!("send_diagnostics(module_path: {:?})", module_path.debug(db));
        }
        let diagnostic_sheet = db.diagnostic_sheet(module_path);
        let diagnostics: Vec<lsp_types::Diagnostic> = diagnostic_sheet
            .diagnostic_iter(db)
            .map(|diagnostic| diagnostic.into())
            .collect();
        if DEBUG_SEND_DIAGNOSTICS {
            eprintln!("before send_flag is set");
        }
        let send_flag = match self.diagnostics_sent.entry(module_path) {
            Entry::Occupied(mut entry) => {
                let is_same = entry.get() == &diagnostics;
                match is_same {
                    true => false,
                    false => {
                        entry.insert(diagnostics.clone());
                        true
                    }
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(diagnostics.clone());
                true
            }
        };
        if DEBUG_SEND_DIAGNOSTICS {
            eprintln!("after send_flag is set");
        }
        if send_flag {
            let Ok(module_diff_path) = module_path.diff_path(db) else { todo!() };
            let Ok(path) = &module_diff_path.abs_path(db) else { todo!() };
            match url_from_diff_path(path) {
                Ok(url) => self.send_diagnostics_aux(url, diagnostics, None),
                Err(_) => eprintln!("error in translating path {:?}", path),
            }
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
                todo!();
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
