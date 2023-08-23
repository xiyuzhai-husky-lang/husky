use crate::*;

use std::ops::FromResidual;
use trackable::{TrackableAtom, TrackableMap, TrackableVec};

impl Devtime {
    pub(crate) fn update(&mut self) {
        todo!()
        // match self.try_update() {
        //     Ok(()) => (),
        //     Err(e) => match e.variant() {
        //         __VMErrorVariant::Normal => todo!(),
        //         __VMErrorVariant::FromBatch { sample_id } => {
        //             self.state.update_presentation(|presentation| {
        //                 presentation.set_specific(SampleId(sample_id))
        //             });
        //             self.update()
        //         }
        //     },
        // }
    }

    fn try_update(&mut self) -> __VMResult<()> {
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }
}
