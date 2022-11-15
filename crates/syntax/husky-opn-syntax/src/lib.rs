mod binary;
mod list;
mod prefix;
mod suffix;

pub use binary::*;
pub use list::*;
pub use prefix::*;
pub use suffix::*;

use husky_text::RangedCustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawOpnVariant {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    CurlBracketed,
    List(ListOpr),
    Field(Option<RangedCustomIdentifier>),
    Abstraction,
}
