use common::*;

use crate::{lsp_ext, server_capabilities, utils::from_json, ServerConfig};

pub fn init_connection(connection: &lsp_server::Connection) -> Result<ServerConfig> {
    // let mut server_capabilities = ServerCapabilities::default();
    // server_capabilities.definition_provider = Some(lsp_types::OneOf::Left(true));
    // connection.initialize(serde_json::to_value(&server_capabilities).unwrap())?;
    // let cfg = ServerConfig {};
    // todo!()
    let (init_id, init_params) = connection.initialize_start()?;
    tracing::info!("InitializeParams: {}", init_params);
    let init_params = from_json::<lsp_types::InitializeParams>("InitializeParams", init_params)?;

    let config = ServerConfig::new(init_params)?;

    connection.initialize_finish(init_id, get_init_result(&config))?;

    Ok(config)
}

fn get_init_result(config: &ServerConfig) -> serde_json::Value {
    let init_result = lsp_types::InitializeResult {
        capabilities: server_capabilities::get_server_capabilities(),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("husky-lang-server"),
            version: None,
        }),
        offset_encoding: if lsp_ext::supports_utf8(&config.client_capabilities) {
            Some("utf-8".to_string())
        } else {
            None
        },
    };
    serde_json::to_value(init_result).unwrap()
}
