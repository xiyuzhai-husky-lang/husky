//! Project loading & configuration updates

use crate::{lsp_ext, server::Server};

impl Server {
    pub(crate) fn current_status(&self) -> lsp_ext::ServerStatusParams {
        let status = lsp_ext::ServerStatusParams {
            health: lsp_ext::Health::Ok,
            message: None,
        };
        status
    }
}
