//! Daniel Jackson is a character in Stargate SG1.
//!
//! He is a linguist.
//!
//! So this visualization serves mainly for linguistics.
pub mod action;

use husky_task_interface::{
    pedestal::IsPedestalFull,
    val_repr::{ValDomainReprInterface, ValReprInterface},
};
use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DanielJacksonFigure;

impl<Pedestal: IsPedestalFull> IsFigure<Pedestal> for DanielJacksonFigure {
    fn new_specific(
        _followed_visual: Option<(TraceId, ValReprInterface, ValDomainReprInterface)>,
        _accompanyings: &[(TraceId, ValReprInterface)],
        _f: impl FnMut(ValReprInterface, &mut VisualSynchrotron) -> Visual,
        _visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        todo!()
    }

    fn new_generic(
        _followed_visual: Option<(TraceId, ValReprInterface, ValDomainReprInterface)>,
        _accompanyings: &[(TraceId, ValReprInterface)],
        _pedestals: impl Iterator<Item = Pedestal>,
        _f: impl FnMut(ValReprInterface, Pedestal, &mut VisualSynchrotron) -> Visual,
        _visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        todo!()
    }
}
