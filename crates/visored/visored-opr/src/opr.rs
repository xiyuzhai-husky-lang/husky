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
    pub const EQ: Self = Self::Binary(VdBaseBinaryOpr::Eq);
}

impl VdBaseOpr {
    pub fn latex_code(self) -> &'static str {
        match self {
            VdBaseOpr::Prefix(opr) => opr.latex_code(),
            VdBaseOpr::Suffix(opr) => opr.latex_code(),
            VdBaseOpr::Binary(opr) => opr.latex_code(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VdCompositeOpr {
    Prefix(VdCompositePrefixOpr),
    Suffix(VdCompositeSuffixOpr),
    Binary(VdCompositeBinaryOpr),
}
