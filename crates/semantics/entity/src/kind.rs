mod morphism;
mod routine;
mod ty;

pub use morphism::*;
pub use routine::*;
use semantics_lazy::LazyStmt;
pub use ty::*;

use crate::*;
use entity_route::InputPlaceholder;
use entity_route::RangedScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Main(Main),
    Module {},
    Feature {
        ty: RangedScope,
        lazy_stmts: Arc<Vec<Arc<LazyStmt>>>,
    },
    Pattern {},
    Func {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedScope,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Ty(TyDefn),
}
