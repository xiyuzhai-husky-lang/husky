use super::*;
use sycamore::prelude::Signalable;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub usize);

impl Signalable for TraceId {}
