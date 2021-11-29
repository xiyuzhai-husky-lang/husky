use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use project_model::Project;
use vfs::AbsPathBuf;

use crate::{
    cli::flags, from_json, run_server, server_capabilities::get_server_capabilities, Result,
    ServerConfig,
};

pub fn init_connection(connection: &Connection) -> Result<ServerConfig> {
    // let mut server_capabilities = ServerCapabilities::default();
    // server_capabilities.definition_provider = Some(lsp_types::OneOf::Left(true));
    // connection.initialize(serde_json::to_value(&server_capabilities).unwrap())?;
    // let cfg = ServerConfig {};
    // todo!()
    let (init_id, init_params) = connection.initialize_start()?;
    tracing::info!("InitializeParams: {}", init_params);
    let init_params =
        crate::from_json::<lsp_types::InitializeParams>("InitializeParams", init_params)?;

    let config = ServerConfig::new(init_params)?;

    connection.initialize_finish(init_id, get_init_result(&config))?;

    Ok(config)
}

fn get_init_result(config: &ServerConfig) -> serde_json::Value {
    let init_result = lsp_types::InitializeResult {
        capabilities: get_server_capabilities(&config),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("rust-analyzer"),
            version: None,
        }),
        offset_encoding: None,
    };
    serde_json::to_value(init_result).unwrap()
}
