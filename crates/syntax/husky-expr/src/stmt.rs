use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type StmtArena = Arena<Stmt>;
pub type StmtIdx = ArenaIdx<Stmt>;
pub type StmtRange = ArenaIdxRange<Stmt>;
pub type StmtMap<V> = ArenaMap<Stmt, V>;

pub enum Stmt {}
