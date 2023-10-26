use crate::*;
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "usize", into = "usize")]
pub struct TraceId(ShiftedU32);

impl TraceId {
    pub(crate) fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.into()
    }

    #[cfg(feature = "mock")]
    pub fn new_mocks(iter: impl IntoIterator<Item = usize>) -> Vec<Self> {
        iter.into_iter()
            .map(|index| Self::from_index(index))
            .collect()
    }
}

impl From<usize> for TraceId {
    fn from(value: usize) -> Self {
        Self(value.into())
    }
}

impl Into<usize> for TraceId {
    fn into(self) -> usize {
        self.0.into()
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Vec<TraceId> {
//     start: TraceId,
//     end: TraceId,
// }

// impl Vec<TraceId> {
//     pub(crate) fn new(start: TraceId, end: TraceId) -> Self {
//         Self { start, end }
//     }

//     #[cfg(feature = "mock")]
//     pub(crate) fn new_mock(start: usize, end: usize) -> Self {
//         Self {
//             start: TraceId::from_index(start),
//             end: TraceId::from_index(end),
//         }
//     }

//     pub fn start(&self) -> TraceId {
//         self.start
//     }

//     pub fn end(&self) -> TraceId {
//         self.end
//     }

//     pub fn into_iter(self) -> impl Iterator<Item = TraceId> {
//         (self.start.index()..self.end.index())
//             .into_iter()
//             .map(|index| TraceId::from_index(index))
//     }
// }
