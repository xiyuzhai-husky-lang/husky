pub mod caryatid;
pub mod chart;

use self::caryatid::*;
use husky_standard_linket_impl::{pedestal::StandardPedestal, static_var::StandardStaticVarId};
use husky_standard_visual_protocol::figure::StandardFigure;
use husky_trace_protocol::{figure::IsFigure, protocol::IsTraceProtocol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct StandardTraceProtocol;

impl Default for StandardTraceProtocol {
    fn default() -> Self {
        Self
    }
}

impl IsTraceProtocol for StandardTraceProtocol {
    type Pedestal = StandardPedestal;

    type Caryatid = StandardCaryatid;

    type Figure = StandardFigure;
}
