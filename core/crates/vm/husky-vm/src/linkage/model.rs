use std::panic::{RefUnwindSafe, UnwindSafe};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ModelLinkage(pub &'static dyn ModelDyn);

impl std::ops::Deref for ModelLinkage {
    type Target = dyn ModelDyn;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq for ModelLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const dyn ModelDyn as *const () as usize
            == other.0 as *const dyn ModelDyn as *const () as usize
    }
}

impl Eq for ModelLinkage {}

pub trait Model: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    type Internal;
    fn train(
        branch_indicator: Option<&dyn std::any::Any>,
        opds: &dyn std::any::Any,
    ) -> Self::Internal;
    fn eval<'eval>(
        internal: &Self::Internal,
        arguments: &dyn std::any::Any,
    ) -> EvalValueResult<'eval> {
        todo!()
    }
}

pub trait ModelDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn train(
        &self,
        branch_indicator: Option<&dyn std::any::Any>,
        opds: &dyn std::any::Any,
    ) -> __EvalResult;
    fn eval<'eval>(
        &self,
        internal: &EvalValue<'static>,
        arguments: &[EvalValue<'eval>],
    ) -> EvalValueResult<'eval>;
}

impl<T> ModelDyn for T
where
    T: Model,
{
    fn train(
        &self,
        branch_indicator: Option<&dyn std::any::Any>,
        opds: &dyn std::any::Any,
    ) -> __EvalResult {
        todo!()
    }

    fn eval<'eval>(
        &self,
        internal: &EvalValue<'static>,
        arguments: &[EvalValue<'eval>],
    ) -> EvalValueResult<'eval> {
        todo!()
    }
}

// pub train: fn(Option<&dyn std::any::Any>, &dyn std::any::Any) -> __EvalResult,
// pub eval: for<'eval> fn(&EvalValue<'static>, Vec<EvalValue<'eval>>) -> EvalValueResult<'eval>, //ugly

// impl PartialEq for ModelLinkage {
//     fn eq(&self, other: &Self) -> bool {
//         (self.train as usize) == (other.train as usize)
//             && (self.eval as usize) == (other.eval as usize)
//     }
// }

// impl Eq for ModelLinkage {}

// impl std::fmt::Debug for ModelLinkage {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
