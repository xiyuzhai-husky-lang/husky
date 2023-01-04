use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type StmtArena = Arena<Stmt>;
pub type StmtIdx = ArenaIdx<Stmt>;
pub type StmtIdxRange = ArenaIdxRange<Stmt>;
pub type StmtMap<V> = ArenaMap<Stmt, V>;

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Let {},
}
