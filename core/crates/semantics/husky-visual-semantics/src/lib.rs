use avec::Avec;
use husky_eager_semantics::FuncStmt;
use husky_lazy_semantics::LazyStmt;
use husky_visual_syntax::StaticVisualizer;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizerSource {
    Static(&'static StaticVisualizer),
    Custom { stmts: Avec<LazyStmt> },
}
