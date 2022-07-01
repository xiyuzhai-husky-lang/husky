mod func;
mod lazy;
mod proc;

pub use func::*;
pub use lazy::*;
pub use proc::*;

use husky_eager_semantics::FuncStmt;
use husky_file::FilePtr;
use husky_lazy_semantics::*;
use husky_text::{TextRange, TextRanged};
use std::sync::Arc;

use crate::{eval_id::FeatureEvalId, unique_allocate::FeatureInterner, *};
