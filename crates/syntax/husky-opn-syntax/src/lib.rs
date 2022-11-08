mod binary;
mod list;
mod prefix;
mod suffix;

pub use binary::*;
pub use list::*;
pub use prefix::*;
pub use suffix::*;

use husky_text::RangedCustomIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub enum RawOpnVariant {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    CurlBracketed,
    List(ListOpr),
    Field(Option<RangedCustomIdentifier>),
    Abstraction,
}

impl std::fmt::Debug for RawOpnVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawOpnVariant::Binary(arg0) => {
                f.write_str("Binary ")?;
                arg0.fmt(f)
            }
            RawOpnVariant::Prefix(arg0) => {
                f.write_str("Prefix ")?;
                arg0.fmt(f)
            }
            RawOpnVariant::Suffix(arg0) => {
                f.write_str("Suffix ")?;
                arg0.fmt(f)
            }
            RawOpnVariant::CurlBracketed => todo!(),
            RawOpnVariant::List(arg0) => {
                f.write_str("List ")?;
                arg0.fmt(f)
            }
            RawOpnVariant::Field(opt_field_ident) => {
                write!(f, "Field {:?}", opt_field_ident)
            }
            RawOpnVariant::Abstraction => todo!(),
        }
    }
}
