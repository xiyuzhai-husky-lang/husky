pub use crate::{FromValue as __FromValue, IntoValue as __IntoValue, Value as __Value};
pub use husky_devsoul_interface::ugly::*;
pub use husky_linkage_impl::standard::ugly::*;
pub use husky_standard_devsoul_interface::ugly::*;

use husky_devsoul_interface::DevEvalContext;
use husky_linkage_impl::standard::StandardLinkageImpl;

pub type __DevEvalContext = DevEvalContext<__LinkageImpl>;
pub type __LinkageImpl = StandardLinkageImpl<__Pedestal>;
pub type __ClosedPoint = __InputId;
// ad hoc
pub type __ValueResult = Result<__Value, ()>;
