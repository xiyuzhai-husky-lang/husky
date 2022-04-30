use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Fast,
    TrackMutation,
    TrackHistory,
}
