mod runtime_storage;

use self::runtime_storage::*;
use husky_hir_deps::HirDepsDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{linkage::IsLinkage, visual::IsVisualProtocol, IsDevAscension, IsTask};
use std::marker::PhantomData;

pub struct MalamuteTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    _marker: PhantomData<(ComptimeDb, VisualProtocol)>,
}

impl<ComptimeDb, VisualProtocol> IsTask for MalamuteTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    type DevAscension = MalamuteDevAscension<ComptimeDb, VisualProtocol>;
}

pub struct MalamuteDevAscension<ComptimeDb, VisualProtocol>(
    PhantomData<(ComptimeDb, VisualProtocol)>,
)
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol;

impl<ComptimeDb, VisualProtocol> IsDevAscension for MalamuteDevAscension<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    type Base = DevInput;

    type LinkTime = MonoLinkTime<ComptimeDb, MalamuteLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeTaskSpecificConfig = ();

    type VisualProtocol = VisualProtocol;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DevInput {
    Train(),
    Val(),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MalamuteLinkage {}

impl IsLinkage for MalamuteLinkage {
    type Value = RegularValue;

    fn eval_fn() -> Self::Value {
        todo!()
    }

    fn eval_gn() -> Self::Value {
        todo!()
    }
}
