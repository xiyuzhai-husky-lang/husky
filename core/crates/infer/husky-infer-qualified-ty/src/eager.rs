mod value;
mod variable;

pub use value::*;
pub use variable::*;

use crate::*;
use husky_entity_route::EntityRouteKind;
use husky_print_utils::msg_once;
use husky_test_utils::{TestDisplay, TestDisplayConfig};
use infer_decl::DeclQueryGroup;
use infer_error::*;
use std::fmt::Write;
use word::RootIdentifier;
