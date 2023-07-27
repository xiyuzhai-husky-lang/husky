use super::*;

pub enum StmtHir {}

pub type StmtHirArena = Arena<StmtHir>;
pub type StmtHirIdx = ArenaIdx<StmtHir>;
pub type StmtHirIdxRange = ArenaIdxRange<StmtHir>;
