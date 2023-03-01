use crate::*;

pub enum RuntimeStmt {}

pub type RuntimeStmtIdx = ArenaIdx<RuntimeStmt>;
pub type RuntimeStmtArena = Arena<RuntimeStmt>;
pub type RuntimeStmtIdxRange = ArenaIdxRange<RuntimeStmt>;
