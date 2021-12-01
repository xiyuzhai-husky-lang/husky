//! See [RequestDispatcher].
use std::{fmt, panic, thread};

use serde::{de::DeserializeOwned, Serialize};

use crate::{server::Server, server_snapshot::ServerSnapshot, task::Task, Result};

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
}

impl<'a> RequestDispatcher<'a> {
    /// Dispatches the request onto the current thread, given full access to
    /// mutable global state. Unlike all other methods here, this one isn't
    /// guarded by `catch_unwind`, so, please, don't make bugs :-)
    pub(crate) fn on_sync_mut<R>(
        &mut self,
        f: fn(&mut Server, R::Params) -> Result<R::Result>,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        todo!()
    }

    /// Dispatches the request onto the current thread.
    pub(crate) fn on_sync<R>(
        &mut self,
        f: fn(ServerSnapshot, R::Params) -> Result<R::Result>,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        todo!()
    }

    /// Dispatches the request onto thread pool
    pub(crate) fn on<R>(
        &mut self,
        f: fn(ServerSnapshot, R::Params) -> Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug + 'static,
        R::Result: Serialize + 'static,
    {
        todo!()
    }

    pub(crate) fn finish(&mut self) {
        todo!()
    }

    fn ParseResult<R>(&mut self) -> Option<(lsp_server::RequestId, R::Params, String)>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + fmt::Debug + 'static,
    {
        todo!()
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
    todo!()
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
    todo!()
}

pub(crate) struct NotificationDispatcher<'a> {
    pub(crate) notif: Option<lsp_server::Notification>,
    pub(crate) server: &'a mut Server,
}

impl<'a> NotificationDispatcher<'a> {
    pub(crate) fn on<N>(&mut self, f: fn(&mut Server, N::Params) -> Result<()>) -> Result<&mut Self>
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
            Err(notif) => {
                self.notif = Some(notif);
                return Ok(self);
            }
        };
        f(self.server, params)?;
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
