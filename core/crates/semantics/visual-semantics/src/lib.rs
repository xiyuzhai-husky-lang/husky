use avec::Avec;
use semantics_eager::FuncStmt;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizerSource {
    Static(StaticVisualizer),
    Custom { stmts: Avec<FuncStmt> },
}
