// #![allow(unused, dead_code, warnings)]
mod capabilities;
pub mod cli;
mod config;
mod convert;
mod init_connection;
mod line_collection;
mod lsp_ext;
mod run_server;
mod semantic_tokens;
mod server;
mod server_capabilities;

pub use config::ServerConfig;

pub use crate::init_connection::init_connection;
pub use crate::run_server::run_server;

pub mod utils;
