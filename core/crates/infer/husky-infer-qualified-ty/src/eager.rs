mod value;
mod variable;

pub use value::*;
pub use variable::*;

use crate::*;
use husky_entity_route::EntityRouteKind;
use infer_decl::DeclQueryGroup;
use infer_error::*;
use print_utils::msg_once;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;
