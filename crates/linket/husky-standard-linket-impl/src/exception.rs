use crate::*;
use husky_linket_impl::exception::TrackedException;

pub type StandardTrackedException = TrackedException<Exception, StandardPedestal>;
pub type StandardTrackedExcepted<T> = Result<T, TrackedException<Exception, StandardPedestal>>;
