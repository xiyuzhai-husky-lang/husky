use avec::Avec;
use semantics_eager::FuncStmt;
use semantics_lazy::LazyStmt;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizerSource {
    Static(&'static StaticVisualizer),
    Custom { stmts: Avec<LazyStmt> },
}
