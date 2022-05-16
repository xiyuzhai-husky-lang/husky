mod binary;
mod list;
mod prefix;
mod suffix;

pub use binary::*;
pub use list::*;
pub use prefix::*;
pub use suffix::*;

use crate::*;
use entity_route::EntityRoutePtr;
use text::RangedCustomIdentifier;

#[derive(PartialEq, Eq, Clone)]
pub enum Opr {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    List(ListOpr),
}

impl std::fmt::Debug for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(arg0) => {
                f.write_str("Binary ")?;
                arg0.fmt(f)
            }
            // Self::Assign(opt_binary) => {
            //     f.write_str("Assign ")?;
            //     if let Some(binary) = opt_binary {
            //         binary.fmt(f)
            //     } else {
            //         Ok(())
            //     }
            // }
            Self::Prefix(arg0) => {
                f.write_str("Prefix ")?;
                arg0.fmt(f)
            }
            Self::Suffix(arg0) => {
                f.write_str("Suffix ")?;
                arg0.fmt(f)
            }
            Self::List(arg0) => {
                f.write_str("List ")?;
                arg0.fmt(f)
            }
        }
    }
}
