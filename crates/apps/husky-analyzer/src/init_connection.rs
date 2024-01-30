use crate::server_capabilities;

pub fn init_connection(connection: &lsp_server::Connection) -> husky_error_utils::Result<()> {
    let (init_id, _init_params) = connection.initialize_start()?;
    connection.initialize_finish(init_id, get_init_result())?;

    Ok(())
}

fn get_init_result() -> serde_json::Value {
    let init_result = lsp_types::InitializeResult {
        capabilities: server_capabilities::get_server_capabilities(),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("husky-lang-server"),
            version: None,
        }),
        offset_encoding: None,
    };
    serde_json::to_value(init_result).unwrap()
}
