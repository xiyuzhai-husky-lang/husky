use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use crate::{cli::flags, from_json, initialize_connection, run_server, Result, ServerConfig};

pub fn initialize_connection(connection: &mut Connection) -> Result<ServerConfig> {
    let mut server_capabilities = ServerCapabilities::default();
    server_capabilities.definition_provider = Some(lsp_types::OneOf::Left(true));
    connection.initialize(serde_json::to_value(&server_capabilities).unwrap())?;
    let cfg = ServerConfig {};
    todo!()
}
