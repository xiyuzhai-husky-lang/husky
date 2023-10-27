pub mod cache;
#[cfg(feature = "client")]
pub mod client;
pub mod id;
pub mod id_map;
#[cfg(feature = "message")]
mod message;
#[cfg(feature = "server")]
pub mod server;
pub mod settings;
pub mod view;

use self::cache::*;
use self::id::*;
use serde::{Deserialize, Serialize};

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}
