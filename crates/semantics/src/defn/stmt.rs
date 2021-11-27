pub enum Stmt {
  Basic(BasicStmt),
  Branch(BranchStmt),
  Loop(LoopStmt),
}
pub enum BasicStmt {
  Break,
  Exec,
  Init,
  Ret,
}
pub enum BranchStmt {
  If,
  Switch,
}
pub enum LoopStmt {
  For,
  ForExt,
  Indefinite,
}
