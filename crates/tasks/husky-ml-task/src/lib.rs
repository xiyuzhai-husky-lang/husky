mod runtime_storage;

use self::runtime_storage::*;
use husky_hir_deps::HirDepsDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{linkage::IsLinkage, visual::IsVisualProtocol, IsDevAscension, IsTask};
use std::marker::PhantomData;

pub struct MlTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    _marker: PhantomData<(ComptimeDb, VisualProtocol)>,
}

impl<ComptimeDb, VisualProtocol> IsTask for MlTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    type DevAscension = MlDevAscension<ComptimeDb, VisualProtocol>;
}

pub struct MlDevAscension<ComptimeDb, VisualProtocol>(PhantomData<(ComptimeDb, VisualProtocol)>)
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol;

impl<ComptimeDb, VisualProtocol> IsDevAscension for MlDevAscension<ComptimeDb, VisualProtocol>
where
    ComptimeDb: HirDepsDb,
    VisualProtocol: IsVisualProtocol,
{
    type Base = DevInput;

    type LinkTime = MonoLinkTime<ComptimeDb, MlLinkage>;

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
