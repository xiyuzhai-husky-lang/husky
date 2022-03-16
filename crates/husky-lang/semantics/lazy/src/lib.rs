mod expr;
mod stmt;
mod variable;

pub use expr::*;
pub use stmt::*;
pub use variable::LazyVariable;

use semantics_infer::InferQueryGroup;
