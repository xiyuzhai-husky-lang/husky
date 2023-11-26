//! See [RequestDispatcher].
use crate::{
    db::{AnalyzerDB, AnalyzerDBSnapshot},
    server::{Server, TaskSet},
    utils::from_json,
    *,
};
use lsp_server::ExtractError;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, panic, thread};

/// A visitor for routing a raw JSON request to an appropriate handler function.
///
/// Most requests are read-only and async and are handled on the threadpool
/// (`on` method).
///
/// Some read-only requests are latency sensitive, and are immediately handled
/// on the main loop thread (`on_sync`). These are typically typing-related
/// requests.
///
/// Some requests modify the state, and are run on the main thread to get
/// `&mut` (`on_sync_mut`).
///
/// Read-only requests are wrapped into `catch_unwind` -- they don't modify the
/// state, so it's OK to recover from their failures.
pub(crate) struct RequestDispatcher<'a> {
    pub(crate) req: Option<lsp_server::Request>,
    pub(crate) server: &'a mut Server,
    pub(crate) control_signal: TaskSet,
}

impl<'a> RequestDispatcher<'a> {
    /// Dispatches the request onto the current thread.
    pub(crate) fn on_sync<R>(
        &mut self,
        f: fn(AnalyzerDBSnapshot, R::Params) -> Result<R::Result>,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        let (id, params, _panic_context) = match self.parse::<R>() {
            Some(it) => it,
            None => return Ok(self),
        };
        let snapshot = self.server.db.snapshot();
        // let result = panic::catch_unwind(move || {
        //     let _pctx = error_utils::panic_context::enter(panic_context);
        //     f(snapshot, params)
        // });
        // ad hoc: avoid catch_unwind for debugging purposes
        let result = Ok(f(snapshot, params));
        let response = thread_result_to_response::<R>(id, result);

        self.server.client_comm.respond(response);
        Ok(self)
    }

    /// Dispatches the request onto the current thread.
    pub(crate) fn on_control<R>(
        &mut self,
        f: fn(AnalyzerDBSnapshot, R::Params) -> TaskSet,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        let (_id, params, _panic_context) = match self.parse::<R>() {
            Some(it) => it,
            None => return Ok(self),
        };

        let snapshot = self.server.db.snapshot();
        // match panic::catch_unwind(move || {
        //     let _pctx = error_utils::panic_context::enter(panic_context);
        //     f(snapshot, params)
        // }) {
        //     Ok(control_signal) => self.control_signal = control_signal,
        //     Err(_) => todo!(),
        // }
        // ad hoc: avoid catch_unwind for debugging purposes
        self.control_signal = f(snapshot, params);
        Ok(self)
    }

    /// Dispatches the request onto thread pool
    pub(crate) fn on<R>(
        &mut self,
        f: fn(AnalyzerDBSnapshot, R::Params) -> Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        let (id, params, _panic_context) = match self.parse::<R>() {
            Some(it) => it,
            None => return self,
        };
        self.server.threadpool.execute({
            let snapshot = self.server.db.snapshot();
            let sender = self.server.event_loop_comm.sender.clone();
            move || {
                // let result = panic::catch_unwind(move || {
                //     let _pctx = error_utils::panic_context::enter(panic_context);
                //     f(snapshot, params)
                // });
                let result = Ok(f(snapshot, params));
                let response = thread_result_to_response::<R>(id, result);
                sender.send(TaskSet::Respond(response)).expect("ok");
            }
        });

        self
    }

    pub(crate) fn finish(&mut self) {
        if let Some(req) = self.req.take() {
            tracing::error!("unknown request: {:?}", req);
            todo!("req = {:?}", req);
            let response = lsp_server::Response::new_err(
                req.id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                "unknown request".to_string(),
            );
            self.server.client_comm.respond(response);
        }
    }

    fn parse<R>(&mut self) -> Option<(lsp_server::RequestId, R::Params, String)>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + fmt::Debug + 'static,
    {
        let req = match self.req {
            Some(ref req) if req.method == R::METHOD => self.req.take().unwrap(),
            _ => return None,
        };

        let res = from_json(R::METHOD, req.params);
        match res {
            Ok(params) => {
                let panic_context =
                    format!("\nversion: {}\nrequest: {} {:#?}", "???", R::METHOD, params);
                Some((req.id, params, panic_context))
            }
            Err(err) => {
                let response = lsp_server::Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::InvalidParams as i32,
                    err.to_string(),
                );
                self.server.client_comm.respond(response);
                None
            }
        }
    }
}

fn thread_result_to_response<R>(
    id: lsp_server::RequestId,
    result: thread::Result<Result<R::Result>>,
) -> lsp_server::Response
where
    R: lsp_types::request::Request + 'static,
    R::Params: DeserializeOwned + 'static,
    R::Result: Serialize + 'static,
{
    match result {
        Ok(result) => result_to_response::<R>(id, result),
        Err(panic) => {
            let mut message = "server panicked".to_string();

            let panic_message = panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied());

            if let Some(panic_message) = panic_message {
                message.push_str(": ");
                message.push_str(panic_message)
            };

            lsp_server::Response::new_err(id, lsp_server::ErrorCode::InternalError as i32, message)
        }
    }
}

fn result_to_response<R>(
    id: lsp_server::RequestId,
    result: Result<R::Result>,
) -> lsp_server::Response
where
    R: lsp_types::request::Request + 'static,
    R::Params: DeserializeOwned + 'static,
    R::Result: Serialize + 'static,
{
    match result {
        Ok(resp) => lsp_server::Response::new_ok(id, &resp),
        Err(e) => {
            p!(e);
            todo!()
        }
    }
}

pub(crate) struct NotificationDispatcher<'a> {
    pub(crate) notif: Option<lsp_server::Notification>,
    pub(crate) server: &'a mut Server,
    pub(crate) task: TaskSet,
}

impl<'a> NotificationDispatcher<'a> {
    pub(crate) fn on_sync<N>(
        &mut self,
        f: fn(&mut Server, N::Params) -> Result<TaskSet>,
    ) -> Result<&mut Self>
    where
        N: lsp_types::notification::Notification + 'static,
        N::Params: DeserializeOwned + Send + 'static,
    {
        let notif = match self.notif.take() {
            Some(it) => it,
            None => return Ok(self),
        };
        let params = match notif.extract::<N::Params>(N::METHOD) {
            Ok(it) => it,
            Err(notif) => match notif {
                ExtractError::MethodMismatch(notif) => {
                    self.notif = Some(notif);
                    return Ok(self);
                }
                ExtractError::JsonError { method, error } => {
                    panic!("Invalid request\nMethod: {method}\n error: {error}",)
                }
            },
        };
        self.task.then(f(self.server, params)?);
        Ok(self)
    }

    pub(crate) fn finish(&mut self) {
        if let Some(notif) = &self.notif {
            if !notif.method.starts_with("$/") {
                tracing::error!("unhandled notification: {:?}", notif);
            }
        }
    }
}
