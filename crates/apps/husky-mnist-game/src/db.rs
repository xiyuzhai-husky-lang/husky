use self::op::{snap::MnistOpSnap, time::OpTime};
use super::*;
use crate::op::frame::{MnistFrame, MnistFramesToBe};
use crate::values::input::Input;
use husky_ml_task_interface::InputId;
use mnist::dataset::MnistDataset;
use shifted_unsigned_int::ShiftedU32;

pub struct MnistDb {
    dataset: MnistDataset,
    input_visuals: Vec<Visual>,
    op_histories: Vec<MnistOpHistory>,
}

pub struct MnistOpHistory {
    op_snaps: Vec<MnistOpSnap>,
}

impl MnistDb {
    pub fn new(visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let dataset = MnistDataset::default();
        let input_visuals = dataset
            .inputs()
            .iter()
            .map(|input| input.visualize(visual_synchrotron))
            .collect();
        let op_snaps = (0..10)
            .into_iter()
            .map(|t| MnistOpSnap::new_ad_hoc(t, visual_synchrotron))
            .collect();
        MnistDb {
            dataset,
            input_visuals,
            op_histories: vec![MnistOpHistory { op_snaps }],
        }
    }
}

/// # getters
impl MnistDb {
    pub fn frames(&self, input_id: InputId) -> &[MnistOpSnap] {
        self.op_history(input_id).op_snaps.as_ref()
    }

    pub(crate) fn input(&self, input_id: InputId) -> &Input {
        self.dataset.input(input_id)
    }

    pub(crate) fn input_visual(&self, input_id: InputId) -> Visual {
        self.input_visuals[input_id.index()]
    }

    pub fn op_history(&self, input_id: InputId) -> &MnistOpHistory {
        &self.op_histories[input_id.index()]
    }
}

impl std::ops::Index<OpTime> for MnistOpHistory {
    type Output = MnistOpSnap;

    fn index(&self, op_time: OpTime) -> &Self::Output {
        &self.op_snaps[op_time.index()]
    }
}
