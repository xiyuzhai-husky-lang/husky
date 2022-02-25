use crate::*;

impl Trace {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        match self.kind {
            TraceKind::Main(_)
            | TraceKind::FeatureStmt(_)
            | TraceKind::FeatureBranch(_)
            | TraceKind::Input(_)
            | TraceKind::DeclStmt { .. } => None,
            TraceKind::FeatureExpr(_) => Some(SubtracesContainerClass::Call),
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
