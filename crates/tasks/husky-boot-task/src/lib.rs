// use husky_boot_linktime::BootLinkTime;
// use husky_standard_value::RegularValue;
// use husky_task::{ascension::IsDevAscension, IsTask};
// use husky_trace_protocol::protocol::trivial::TrivialTraceProtocol;
// use husky_trivial_linkage_impl::TrivialLinkage;

// #[derive(Default)]
// pub struct BootTask;

// impl IsTask for BootTask {
//     type DevAscension = BootDevAscension;
// }

// pub struct BootDevAscension;

// impl IsDevAscension for BootDevAscension {
//     type Base = ();

//     type Linktime = BootLinkTime<StandardDevComptimeDb, TrivialLinkage>;

//     type Value = RegularValue;

//     type RuntimeStorage = ();

//     type RuntimeSpecificConfig = ();

//     type TraceProtocol = TrivialTraceProtocol;

//     type ComptimeDb = StandardDevComptimeDb;
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum BootLinkage {}
