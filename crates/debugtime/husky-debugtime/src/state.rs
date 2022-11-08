mod old;

pub use old::*;

use crate::*;
use trackable::{
    TrackSimple, Trackable, TrackableAtom, TrackableMakeChangeM, TrackableMap,
    TrackableTakeChangeM, TrackableVec, TrackableVecSimple,
};

pub type DebugtimeState = ServerTraceState<TraceNode>;

pub type DebugtimeOldState = ServerTraceOldState<TraceNode>;

pub type DebugtimeStateChange = ServerTraceStateChange;
