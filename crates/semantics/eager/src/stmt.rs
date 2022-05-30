mod func;
mod parser;
mod proc;

pub use func::*;
pub use parser::*;
pub use proc::*;

use crate::expr::EagerExprParser;
use crate::*;
use ast::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use print_utils::*;
use semantics_error::{err, not_none};
use text::TextRange;
use word::{CustomIdentifier, RootIdentifier};
