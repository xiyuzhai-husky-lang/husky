mod runtime_storage;

use self::runtime_storage::*;
use husky_hir_deps::HirDepsDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{IsDevAscension, IsLinkage, IsTask};
use std::{
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub struct MlTask<ComptimeDb>
where
    ComptimeDb: HirDepsDb + UnwindSafe + RefUnwindSafe,
{
    _marker: PhantomData<(ComptimeDb)>,
}

impl<ComptimeDb> IsTask for MlTask<ComptimeDb>
where
    ComptimeDb: HirDepsDb + UnwindSafe + RefUnwindSafe,
{
    type DevAscension = MlDevAscension<ComptimeDb>;
}

pub struct MlDevAscension<ComptimeDb>(PhantomData<(ComptimeDb)>)
where
    ComptimeDb: HirDepsDb + UnwindSafe + RefUnwindSafe;

impl<ComptimeDb> IsDevAscension for MlDevAscension<ComptimeDb>
where
    ComptimeDb: HirDepsDb + UnwindSafe + RefUnwindSafe,
{
    type Base = DevInput;

    type LinkTime = MonoLinkTime<ComptimeDb, MlLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeTaskSpecificConfig = ();
}

pub enum DevInput {
    Train(),
    Val(),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MlLinkage {}

impl IsLinkage for MlLinkage {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}
