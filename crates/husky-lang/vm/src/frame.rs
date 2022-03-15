use crate::*;
use word::CustomIdentifier;

#[derive(Debug, Clone)]
pub struct LoopFrameSnapshot {
    pub stack: StackSnapshot,
    pub changes: Vec<()>,
    pub control: ControlSnapshot,
    pub frame_var_value: i32,
    pub kind: FrameKind,
}

#[derive(Debug, Clone)]
pub enum FrameKind {
    For(CustomIdentifier),
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
