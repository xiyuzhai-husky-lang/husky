pub mod binary;
pub mod prefix;
pub mod suffix;

use self::{
    binary::{VdBaseBinaryOpr, VdCompositeBinaryOpr},
    prefix::{VdBasePrefixOpr, VdCompositePrefixOpr},
    suffix::{VdBaseSuffixOpr, VdCompositeSuffixOpr},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VdBaseOpr {
    Prefix(VdBasePrefixOpr),
    Suffix(VdBaseSuffixOpr),
    Binary(VdBaseBinaryOpr),
}

impl VdBaseOpr {
    pub const ADD: Self = Self::Binary(VdBaseBinaryOpr::Add);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VdCompositeOpr {
    Prefix(VdCompositePrefixOpr),
    Suffix(VdCompositeSuffixOpr),
    Binary(VdCompositeBinaryOpr),
}
