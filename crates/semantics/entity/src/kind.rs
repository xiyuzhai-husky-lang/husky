pub mod ty;

use semantics_lazy::LazyStmt;
pub use ty::*;

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
    Module {},
    Feature(Vec<FuncStmt>),
    Pattern {},
    Func {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
    Ty(Ty),
}
