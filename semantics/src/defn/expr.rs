use crate::*;
pub enum Expr {
  Opn {
    opn: Opn,
    ovld: OvldID,
    opds: Vec<Expr>,
  },
  Par(Box<Expr>),
  Var,
  Const(ConstExpr),
}
pub enum ConstExpr {
  IntLiteral(i32),
  FloatLiteral(f32),
  BoolLiteral(bool),
  Global(EntityID),
}
