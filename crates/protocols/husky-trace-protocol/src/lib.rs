pub mod cache;
#[cfg(feature = "client")]
pub mod client;
pub mod id;
#[cfg(feature = "server")]
pub mod server;
pub mod settings;
pub mod view;

use self::cache::*;
use self::id::*;

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}
