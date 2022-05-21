mod xml;

pub use xml::*;

use avec::Avec;
use semantics_eager::FuncStmt;
use std::sync::Arc;
use visual_syntax::StaticVisualizer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizerSource {
    Static(StaticVisualizer),
    Xml {
        stmts: Avec<FuncStmt>,
        xml_expr: Arc<XmlExpr>,
    },
}
