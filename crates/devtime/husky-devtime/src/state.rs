mod old;

pub use old::*;

use crate::*;
use trackable::{
    TrackSimple, Trackable, TrackableAtom, TrackableMakeChangeM, TrackableMap,
    TrackableTakeChangeM, TrackableVec, TrackableVecSimple,
};

pub type HuskyDevtimeState = ServerTraceState<TraceNode>;

pub type HuskyDevtimeOldState = ServerTraceOldState<TraceNode>;

pub type HuskyDevtimeStateChange = ServerTraceStateChange;
