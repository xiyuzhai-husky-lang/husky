mod runtime_storage;

use self::runtime_storage::*;
use husky_linkage_path::db::LinkagePathDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{linkage::IsLinkage, IsDevAscension, IsTask};
use husky_val::ValDb;
use husky_visual_protocol::IsVisualProtocol;
use std::marker::PhantomData;

pub struct MlTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: ValDb,
    VisualProtocol: IsVisualProtocol,
{
    _marker: PhantomData<(ComptimeDb, VisualProtocol)>,
}

impl<ComptimeDb, VisualProtocol> MlTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: ValDb,
    VisualProtocol: IsVisualProtocol,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<ComptimeDb, VisualProtocol> IsTask for MlTask<ComptimeDb, VisualProtocol>
where
    ComptimeDb: Default + husky_vfs::VfsDb + ValDb + LinkagePathDb + Send + 'static,
    VisualProtocol: IsVisualProtocol + Send,
{
    type DevAscension = MlDevAscension<ComptimeDb, VisualProtocol>;
}

pub struct MlDevAscension<ComptimeDb, VisualProtocol>(PhantomData<(ComptimeDb, VisualProtocol)>)
where
    ComptimeDb: ValDb,
    VisualProtocol: IsVisualProtocol;

impl<ComptimeDb, VisualProtocol> IsDevAscension for MlDevAscension<ComptimeDb, VisualProtocol>
where
    ComptimeDb: Default + husky_vfs::VfsDb + ValDb + LinkagePathDb + Send,
    VisualProtocol: IsVisualProtocol,
{
    type Base = DevInput;

    type Linktime = MonoLinkTime<ComptimeDb, MlLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeTaskSpecificConfig = ();

    type VisualProtocol = VisualProtocol;

    type ComptimeDb = ComptimeDb;
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
