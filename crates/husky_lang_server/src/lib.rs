// #![allow(unused, dead_code, warnings)]
mod capabilities;
pub mod cli;
mod config;
mod convert;
mod init_connection;
mod line_index;
mod lsp_error;
mod lsp_ext;
mod lsp_utils;
mod reload;
mod run_server;
mod semantic_tokens;
mod server;
mod server_capabilities;
mod server_snapshot;
mod task;
mod taskpool;

pub use config::ServerConfig;

pub use crate::init_connection::init_connection;
pub use crate::run_server::run_server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

use serde::de::DeserializeOwned;

pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    let res = serde_json::from_value(json.clone())
        .map_err(|e| format!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}
