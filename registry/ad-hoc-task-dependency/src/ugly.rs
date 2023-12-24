pub use crate::{
    FromValue as __FromValue, IntoValue as __IntoValue, TaskJarIndex as __TaskJarIndex,
    TaskJarIndexOnceCell as __TaskJarIndexOnceCell, Value as __Value,
};
use husky_linkage_impl::standard::LinkageImpl;
pub use husky_ml_task_prelude::{
    dev_eval_context as __dev_eval_context, eval_val_repr as __eval_val_repr,
    with_dev_eval_context as __with_dev_eval_context, DEV_EVAL_CONTEXT as __DEV_EVAL_CONTEXT,
};
use husky_ml_task_prelude::{InputId, MlPedestal};
pub use husky_task_prelude::TaskIngredientIndex as __TaskIngredientIndex;

use husky_task_prelude::DevEvalContext;

pub type __DevEvalContext = DevEvalContext<__LinkageImpl>;
pub type __LinkageImpl = LinkageImpl<MlPedestal>;
pub type __ClosedPoint = InputId;
pub use husky_linkage_impl::standard::{
    IsGnItem as __IsGnItem, ValControlFlow as __ValControlFlow,
};
pub use husky_ml_task_prelude::MlPedestal as __Pedestal;
pub use husky_task_prelude::val_repr::{
    ValArgumentReprInterface as __ValArgumentReprInterface, ValReprInterface as __ValReprInterface,
};
// ad hoc
pub type __ValueResult = Result<__Value, ()>;
