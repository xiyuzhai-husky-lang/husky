use husky_dev_comptime::db::ComptimeDb;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{IsDevAscension, IsTask};
use husky_trivial_linkage::TrivialLinkage;
use husky_visual_protocol::trivial::TrivialVisualProtocol;

pub struct BootTask;

impl IsTask for BootTask {
    type DevAscension = BootDevAscension;
}

pub struct BootDevAscension;

impl IsDevAscension for BootDevAscension {
    type Base = ();

    type LinkageTable = MonoLinkTime<ComptimeDb, TrivialLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = ();

    type RuntimeTaskSpecificConfig = ();

    type VisualProtocol = TrivialVisualProtocol;
}

pub struct BootLinkTime;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BootLinkage {}
