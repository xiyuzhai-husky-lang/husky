#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SampleId(pub usize);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Label(pub i32);

impl From<u8> for Label {
    fn from(v: u8) -> Self {
        Self(v as i32)
    }
}
