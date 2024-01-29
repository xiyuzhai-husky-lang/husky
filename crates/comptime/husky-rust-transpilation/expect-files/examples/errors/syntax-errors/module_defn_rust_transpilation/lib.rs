#![allow(warnings, non_snake_case)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};

ad_hoc_task_dependency::init_crate!();

pub mod ast;

pub use self::ast::*;

use malamute::*;
use mnist::*;