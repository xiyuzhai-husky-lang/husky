use self::op::snap::MnistOpSnap;
use super::*;
use crate::op::frame::{MnistFrame, MnistFramesToBe};
use crate::values::input::Input;
use shifted_unsigned_int::ShiftedU32;

pub struct MnistDb {
    entries: Vec<MnistDbEntry>,
}

pub struct MnistDbEntry {
    input: Input,
    op_snaps: Vec<MnistOpSnap>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputId(ShiftedU32);

impl InputId {
    pub(crate) fn from_index(index: usize) -> Self {
        Self(index.into())
    }
}

impl Default for MnistDb {
    fn default() -> Self {
        let op_snaps = (0..10)
            .into_iter()
            .map(|t| MnistOpSnap::new_ad_hoc(t))
            .collect();
        MnistDb {
            entries: vec![MnistDbEntry {
                input: Input::default(),
                op_snaps,
            }],
        }
    }
}

/// # getters
impl MnistDb {
    pub fn frames(&self, input_id: InputId) -> &[MnistOpSnap] {
        self[input_id].op_snaps.as_ref()
    }
}

impl std::ops::Index<InputId> for MnistDb {
    type Output = MnistDbEntry;

    fn index(&self, id: InputId) -> &Self::Output {
        let index: usize = id.0.into();
        &self.entries[index]
    }
}
