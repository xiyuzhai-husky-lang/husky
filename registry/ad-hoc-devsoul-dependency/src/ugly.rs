pub use crate::{FromValue as __FromValue, IntoValue as __IntoValue, Value as __Value};
pub use husky_devsoul_interface::ugly::*;
pub use husky_linket_impl::standard::ugly::*;
pub use husky_standard_devsoul_interface::ugly::*;

use husky_devsoul_interface::DevEvalContext;
use husky_linket_impl::standard::StandardLinketImpl;

pub type __DevEvalContext = DevEvalContext<__LinketImpl>;
pub type __LinketImpl = StandardLinketImpl<__Pedestal>;
// ad hoc
pub type __ValueResult = Result<__Value, ()>;
