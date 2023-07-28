use crate::*;
use husky_coword::Ident;

#[derive(Debug)]
pub struct LoopFrameData {
    pub stack_snapshot: StackSnapshot,
    pub mutations: Vec<MutationData>,
    pub control: ControlSnapshot,
    pub frame_var_value: i32,
    pub frame_kind: FrameKind,
}

#[derive(Debug, Clone)]
pub enum FrameKind {
    For(Ident),
    Loop,
}

impl From<VMLoopKind> for FrameKind {
    fn from(loop_kind: VMLoopKind) -> Self {
        match loop_kind {
            VMLoopKind::For { frame_var, .. } => FrameKind::For(frame_var),
            VMLoopKind::ForExt { frame_var, .. } => FrameKind::For(frame_var),
            VMLoopKind::Loop => FrameKind::Loop,
        }
    }
}
