use idx_arena::{map::ArenaMap, ArenaIdx, ArenaRange, IdxArena};

pub type StmtArena = IdxArena<Stmt>;
pub type StmtIdx = ArenaIdx<Stmt>;
pub type StmtRange = ArenaRange<Stmt>;
pub type StmtMap<V> = ArenaMap<Stmt, V>;

pub enum Stmt {}
