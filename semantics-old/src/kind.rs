mod func;
mod module;
mod pattern;
mod proc;
pub mod ty;

use crate::*;

pub use func::Func;
pub use module::Module;
pub use pattern::Pattern;
pub use proc::Proc;
use scope::InputPlaceholder;
use scope::RangedScope;
pub use ty::Ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Main {
    pub stmts: Arc<Vec<Arc<DeclStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Module(Module),
    Feature(Vec<DeclStmt>),
    Pattern(Pattern),
    Func {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<DeclStmt>>>,
    },
    Proc {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
    Ty(Ty),
}
