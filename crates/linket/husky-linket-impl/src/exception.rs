use crate::{pedestal::IsPedestal, *};
use husky_ki_repr_interface::KiReprInterface;
use husky_value_interface::exception::IsException;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrackedException<E: IsException, P: IsPedestal> {
    exception: E,
    source: ExceptionSource,
    pedestal: P,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ExceptionSource {
    Ki(KiReprInterface),
}
