pub use crate::{FromValue as __FromValue, IntoValue as __IntoValue, Value as __Value};
pub use husky_devsoul_interface::ugly::*;
pub use husky_ki_repr_interface::ugly::*;
pub use husky_standard_devsoul_interface::ugly::*;
pub use husky_standard_linket_impl::ugly::*;

use husky_linket_impl::eval_context::DevEvalContext;
use husky_standard_linket_impl::StandardLinketImpl;

pub type __DevEvalContext = DevEvalContext<__LinketImpl>;
pub type __LinketImpl = StandardLinketImpl;
// ad hoc
pub type __ValueResult = Result<__Value, ()>;
