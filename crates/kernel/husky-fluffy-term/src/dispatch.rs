// provides support for determining expression type

mod dynamic_dispatch;
mod static_dispatch;

pub use self::dynamic_dispatch::*;
pub use self::static_dispatch::*;

use crate::*;
