mod regular;
mod ty_memo;

pub use self::regular::*;
pub use self::ty_memo::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldModifier {
    Pure,
    Mut,
    Const,
    Leashed,
}
