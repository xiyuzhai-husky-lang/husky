use husky_boot_linktime::BootLinkTime;
use husky_dev_comptime::db::DevComptimeDb;
use husky_regular_value::RegularValue;
use husky_task::{IsDevAscension, IsTask};
use husky_trivial_linkage::TrivialLinkage;
use husky_visual_protocol::trivial::TrivialVisualProtocol;

#[derive(Default)]
pub struct BootTask;

impl IsTask for BootTask {
    type DevAscension = BootDevAscension;
}

pub struct BootDevAscension;

impl IsDevAscension for BootDevAscension {
    type Base = ();

    type LinkTime = BootLinkTime<DevComptimeDb, TrivialLinkage>;

    type Value = RegularValue;

    type RuntimeStorage = ();

    type RuntimeTaskSpecificConfig = ();

    type VisualProtocol = TrivialVisualProtocol;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BootLinkage {}
