pub mod binary;
pub mod prefix;
pub mod suffix;

use self::{binary::VdBinaryOpr, prefix::VdPrefixOpr, suffix::VdSuffixOpr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VdOpr {
    Binary(VdBinaryOpr),
    Prefix(VdPrefixOpr),
    Suffix(VdSuffixOpr),
}
