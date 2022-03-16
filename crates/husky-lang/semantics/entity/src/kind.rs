mod func;
mod module;
mod pattern;
mod proc;
pub mod ty;

pub use func::Func;
pub use module::Module;
pub use pattern::Pattern;
pub use proc::Proc;
use semantics_lazy::LazyStmt;
pub use ty::Ty;

use crate::*;
use scope::InputPlaceholder;
use scope::RangedScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Main {
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Main(Main),
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
