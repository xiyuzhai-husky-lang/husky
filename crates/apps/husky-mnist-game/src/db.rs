use self::op::{snap::MnistOpSnap, time::OpTime};
use super::*;
use crate::op::frame::{MnistFrame, MnistFramesToBe};
use crate::values::input::Input;
use husky_ml_task_interface::InputId;
use mnist::dataset::MnistDataset;
use shifted_unsigned_int::ShiftedU32;

pub struct MnistDb {
    dataset: MnistDataset,
    entries: Vec<MnistDbEntry>,
}

pub struct MnistDbEntry {
    op_snaps: Vec<MnistOpSnap>,
}

impl Default for MnistDb {
    fn default() -> Self {
        let mut visual_synchrotron = VisualSynchrotron::default();
        let op_snaps = (0..10)
            .into_iter()
            .map(|t| MnistOpSnap::new_ad_hoc(t, &mut visual_synchrotron))
            .collect();
        MnistDb {
            dataset: MnistDataset::default(),
            entries: vec![MnistDbEntry { op_snaps }],
        }
    }
}

/// # getters
impl MnistDb {
    pub fn frames(&self, input_id: InputId) -> &[MnistOpSnap] {
        self[input_id].op_snaps.as_ref()
    }

    pub(crate) fn input(&self) -> &Input {
        todo!()
    }

    pub(crate) fn op_snap(&self, input_id: InputId, op_time: OpTime) -> &MnistOpSnap {
        todo!()
    }
}

impl std::ops::Index<InputId> for MnistDb {
    type Output = MnistDbEntry;

    fn index(&self, id: InputId) -> &Self::Output {
        &self.entries[id.index()]
    }
}
