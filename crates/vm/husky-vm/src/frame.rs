use crate::*;
use husky_identifier::Identifier;

#[derive(Debug)]
pub struct LoopFrameData<'eval> {
    pub stack_snapshot: StackSnapshot<'eval>,
    pub mutations: Vec<MutationData<'eval>>,
    pub control: ControlSnapshot<'eval>,
    pub frame_var_value: i32,
    pub frame_kind: FrameKind,
}

#[derive(Debug, Clone)]
pub enum FrameKind {
    For(Identifier),
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
