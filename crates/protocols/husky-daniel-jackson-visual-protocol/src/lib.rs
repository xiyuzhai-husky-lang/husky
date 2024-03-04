//! Daniel Jackson is a character in Stargate SG1.
//!
//! He is a linguist.
//!
//! So this visualization serves mainly for linguistics.
pub mod action;

use husky_task_interface::{ki_repr::KiReprInterface, pedestal::IsPedestalFull};
use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DanielJacksonFigure;

impl<Pedestal: IsPedestalFull> IsFigure<Pedestal> for DanielJacksonFigure {
    fn new_specific(
        _followed_visual: Option<(TraceId, KiReprInterface)>,
        _accompanyings: &[(TraceId, KiReprInterface)],
        _f: impl FnMut(KiReprInterface, &mut VisualSynchrotron) -> Visual,
        _visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        todo!()
    }

    fn new_generic(
        _followed_visual: Option<(TraceId, KiReprInterface)>,
        _accompanyings: &[(TraceId, KiReprInterface)],
        _pedestals: impl Iterator<Item = Pedestal>,
        _f: impl FnMut(KiReprInterface, Pedestal, &mut VisualSynchrotron) -> Visual,
        _visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        todo!()
    }
}
