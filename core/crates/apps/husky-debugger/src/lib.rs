mod config;
mod error;
mod gui;
mod instance;
mod internal;
mod mode;
mod notif;

pub use config::HuskyDebuggerConfig;
pub use error::{DebuggerError, DebuggerResult};
pub use mode::Mode;

use avec::Avec;
use futures::executor::ThreadPool;
use gui::handle_query;
use husky_compile_time::HuskyComptime;
use husky_file::FilePtr;
use husky_print_utils::*;
use husky_root_static_defn::__StaticLinkageKey;
use husky_test_utils::TestResult;
use husky_trace_protocol::*;
use husky_trace_time::HuskyTraceTime;
use instance::*;
use internal::HuskyDebuggerInternal;
use json_result::JsonResult;
use notif::handle_notif;
use std::{
    collections::HashMap,
    convert::Infallible,
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{sync::Mutex, time::Instant};
use vm::__Linkage;
use warp::Filter;

pub type GetLinkagesFromCDylib =
    unsafe extern "C" fn() -> &'static [(__StaticLinkageKey, __Linkage)];

// let flags = HuskyDebuggerFlags::from_env().expect("invalid arguments");
// let opt_library: Option<Library> = if let Some(cdylib) = flags.cdylib {
//     Some(unsafe { Library::new(cdylib) }.expect("it should work"))
// } else {
//     None
// };
// let linkages_from_cdylib: &[(__StaticLinkageKey, __Linkage)] = opt_library
//     .as_ref()
//     .map(|library| unsafe {
//         library
//             .get::<GetLinkagesFromCDylib>(b"get_linkages")
//             .expect("what")()
//     })
//     .unwrap_or(&[]);
// let mode: Mode = flags.mode.into();
// let package_dir: PathBuf = flags.package_dir.unwrap().into();
// mode.apply(&package_dir, linkages_from_cdylib).await

pub fn debugger_launch(package_dir: PathBuf, verbose: bool) {
    todo!()
}

pub fn debugger_test(packages_dir: PathBuf, verbose: bool) {
    todo!()
}
