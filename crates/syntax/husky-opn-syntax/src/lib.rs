mod binary;
mod list;
mod prefix;
mod suffix;

pub use binary::*;
pub use list::*;
pub use prefix::*;
pub use suffix::*;

use husky_text::RangedIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opn {
    Binary(BinaryPunctuation),
    Prefix(PrefixPunctuation),
    Suffix(SuffixPunctuation),
    CurlBracketed,
    List(ListOpr),
    Field(Option<RangedIdentifier>),
    Abstraction,
}
