// provides support for determining expression type

pub mod dynamic_dispatch;
pub mod static_dispatch;

pub use self::dynamic_dispatch::*;
pub use self::static_dispatch::*;

use crate::*;
