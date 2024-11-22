pub mod base;
pub mod composite;

use self::{base::*, composite::*};
use super::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSeparatorSignature {
    Base(VdBaseSeparatorSignature),
}
