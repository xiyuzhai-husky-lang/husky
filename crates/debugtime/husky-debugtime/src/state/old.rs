use crate::*;
use husky_entity_route::EntityRoutePtr;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TraceSketch {
    Main,
    Module(EntityRoutePtr),
    EntityFeature(EntityRoutePtr),
}

impl AsTraceSketch for TraceSketch {
    type Node = TraceNode;

    fn new(node: &Self::Node) -> Option<Self> {
        match node.trace().variant {
            TraceVariant::Main(_) => Some(TraceSketch::Main),
            TraceVariant::Module { route, file, range } => Some(TraceSketch::Module(route)),
            TraceVariant::EntityFeature { route, ref repr } => {
                Some(TraceSketch::EntityFeature(route))
            }
            TraceVariant::FeatureStmt(_) => None,
            TraceVariant::FeatureBranch(_) => None,
            TraceVariant::FeatureExpr(_) => None,
            TraceVariant::FeatureCallArgument { name, ref argument } => None,
            TraceVariant::FuncStmt { .. } => None,
            TraceVariant::ProcStmt { .. } => None,
            TraceVariant::ProcBranch { .. } => None,
            TraceVariant::FuncBranch { .. } => None,
            TraceVariant::LoopFrame { .. } => None,
            TraceVariant::EagerExpr { .. } => None,
            TraceVariant::EagerCallArgument { .. } => None,
            TraceVariant::CallHead { .. } => None,
        }
    }
}
