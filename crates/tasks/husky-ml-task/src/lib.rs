mod runtime_storage;

use self::runtime_storage::*;
use husky_linkage_path::db::LinkagePathDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_rust_transpilation::db::RustTranspilationDb;
use husky_task::{ascension::IsDevAscension, linkage::IsLinkage, IsTask};
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_val::ValDb;
use husky_visual_protocol::IsVisualProtocol;
use serde::{Deserialize, Serialize};
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
    ComptimeDb:
        Default + husky_vfs::VfsDb + ValDb + RustTranspilationDb + LinkagePathDb + Send + 'static,
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
    ComptimeDb: Default + husky_vfs::VfsDb + ValDb + RustTranspilationDb + LinkagePathDb + Send,
    VisualProtocol: IsVisualProtocol,
{
    type Base = DevInput;

    type Linktime = MonoLinkTime<ComptimeDb, MlLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = MlTraceProtocol<VisualProtocol>;

    type ComptimeDb = ComptimeDb;
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MlTraceProtocol<VisualProtocol>(VisualProtocol);

impl<VisualProtocol> IsTraceProtocol for MlTraceProtocol<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol,
{
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
