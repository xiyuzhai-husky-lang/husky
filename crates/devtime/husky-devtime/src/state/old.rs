use crate::*;
use husky_entity_route::EntityRoutePtr;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TraceSketch {
    Main,
    Module(EntityRoutePtr),
    EntityFeature(EntityRoutePtr),
}

impl TraceSketch {
    pub fn new(trace_node: &TraceNode) -> Option<Self> {
        match trace_node.trace().variant {
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

#[must_use]
pub struct HuskyDevtimeOldState {
    presentation: Presentation,
    trace_nodes: Vec<TraceNode>,
    trace_id_map: Vec<TraceIdMatch>,
    trace_sketches: HashMap<TraceSketch, TraceId>,
    fixed: bool,
}

impl HuskyDevtimeOldState {
    pub fn new(presentation: Presentation, trace_nodes: Vec<TraceNode>) -> Self {
        let trace_sketches = trace_nodes
            .iter()
            .filter_map(|trace_node| {
                TraceSketch::new(trace_node).map(|sketch| (sketch, trace_node.trace().id()))
            })
            .collect();
        Self {
            presentation,
            trace_nodes,
            trace_id_map: vec![],
            trace_sketches,
            fixed: false,
        }
    }

    pub fn try_match_node(&mut self, new_node: &TraceNode) -> Option<&TraceNode> {
        let new_id = new_node.trace().id();
        assert!(self.try_match_id(new_id).is_none());
        let sketch = TraceSketch::new(new_node)?;
        let old_id = self.trace_sketches.get(&sketch)?;
        let old_node = &self.trace_nodes[old_id.raw()];
        self.trace_id_map.push(TraceIdMatch {
            old_id: *old_id,
            new_id,
        });
        Some(old_node)
    }

    pub fn try_match_id(&self, new: TraceId) -> Option<TraceId> {
        self.trace_id_map
            .iter()
            .find(|m| m.new_id == new)
            .map(|m| m.old_id)
    }

    pub fn fix(&mut self) {
        assert!(!self.fixed);
        self.fixed = true;
    }

    pub fn mimic_presentation(&self, trace_nodes: &[TraceNode]) -> Presentation {
        assert!(self.fixed);
        self.presentation.mimic(&|id| {
            self.trace_id_map
                .iter()
                .find(|m| m.old_id == id)
                .map(|m| &trace_nodes[m.new_id.raw()].trace().raw_data)
        })
    }
}

pub struct TraceIdMatch {
    pub old_id: TraceId,
    pub new_id: TraceId,
}
