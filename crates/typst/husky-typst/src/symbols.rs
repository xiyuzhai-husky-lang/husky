//! Modifiable symbols.

mod emoji;
mod sym;
mod symbol;

pub use self::emoji::*;
pub use self::sym::*;
pub use self::symbol::*;

use crate::foundations::{category, TypstDefnKind, TypstValueAssignmentGroup};

/// These two modules give names to symbols and emoji to make them easy to
/// insert with a normal keyboard. Alternatively, you can also always directly
/// enter Unicode symbols into your text and formulas. In addition to the
/// symbols listed below, math mode defines `dif` and `Dif`. These are not
/// normal symbol values because they also affect spacing and font style.
#[category]
pub static SYMBOLS: TypstDefnKind;

/// Hook up all `symbol` definitions.
pub(super) fn define(global: &mut TypstValueAssignmentGroup) {
    global.category(SYMBOLS);
    global.define_type::<Symbol>();
    global.define_module(sym());
    global.define_module(emoji());
}
