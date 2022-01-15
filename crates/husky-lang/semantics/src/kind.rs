mod def;
mod func;
mod module;
mod proc;
pub mod ty;

use crate::*;

pub use def::Def;
pub use func::Func;
pub use module::Module;
pub use proc::Proc;
pub use ty::Ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Main {
    pub stmts: Vec<LazyStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Module(Module),
    Def(Def),
    Func(Func),
    Proc(Proc),
    Ty(Ty),
}
