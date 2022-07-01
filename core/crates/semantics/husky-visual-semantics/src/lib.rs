use avec::Avec;
use husky_eager_semantics::FuncStmt;
use husky_lazy_semantics::LazyStmt;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizerSource {
    Static(&'static StaticVisualizer),
    Custom { stmts: Avec<LazyStmt> },
}
