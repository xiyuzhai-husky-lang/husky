use self::op::history::{MnistOpHistory, OpTime};
use super::*;
use crate::op::frame::MnistOpFrame;
use mnist::{dataset::MnistDataset, input_id::MnistInputId};

pub struct MnistDb {
    input_visuals: Vec<Visual>,
    op_histories: Vec<MnistOpHistory>,
}

impl MnistDb {
    pub fn new(visual_synchrotron: &mut VisualSynchrotron) -> Self {
        let dataset = MnistDataset::default();
        let input_visuals = dataset
            .inputs()
            .map(|input| input.visualize(visual_synchrotron))
            .collect();
        let input = dataset.input(MnistInputId::from_index(0));
        let op_histories = vec![MnistOpHistory::new(input, visual_synchrotron)];
        MnistDb {
            input_visuals,
            op_histories,
        }
    }
}

/// # getters
impl MnistDb {
    pub fn op_frames(&self, input_id: MnistInputId) -> &[MnistOpFrame] {
        self.op_history(input_id).op_frames()
    }

    pub(crate) fn input_visual(&self, input_id: MnistInputId) -> Visual {
        self.input_visuals[input_id.index()]
    }

    pub fn op_history(&self, input_id: MnistInputId) -> &MnistOpHistory {
        &self.op_histories[input_id.index()]
    }
}

impl std::ops::Index<OpTime> for MnistOpHistory {
    type Output = MnistOpFrame;

    fn index(&self, op_time: OpTime) -> &Self::Output {
        &self.op_frames()[op_time.index()]
    }
}
