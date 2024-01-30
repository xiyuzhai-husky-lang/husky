#![allow(warnings, non_snake_case)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};

ad_hoc_task_dependency::init_crate!();

pub mod ast;
pub mod uses;

pub use self::ast::*;
pub use self::uses::*;

use malamute::*;
use mnist::*;