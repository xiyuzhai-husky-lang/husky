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
use path_utils::collect_all_package_dirs;
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

pub async fn debugger_launch(package_dir: PathBuf, verbose: bool) {
    todo!()
}

pub async fn debugger_test(packages_dir: PathBuf, verbose: bool) {
    assert!(packages_dir.is_dir());
    let package_dirs = collect_all_package_dirs(&packages_dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        husky_print_utils::CYAN,
        husky_print_utils::RESET,
        package_dirs.len()
    );

    for package_dir in package_dirs {
        println!(
            "\n{}test{} {}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            package_dir.as_os_str().to_str().unwrap(),
        );
        let husky_debugger = HuskyDebuggerInstance::new(
            HuskyDebuggerConfig {
                package_dir,
                opt_sample_id: Some(SampleId(23)),
                verbose: false,
                compiled: false,
            },
            &[],
        );
        match husky_debugger
            .serve_on_error("localhost:51617", SampleId(0))
            .await
        {
            TestResult::Success => finalize_success(),
            TestResult::Failure => finalize_failure(),
        }
    }

    fn finalize_success() {
        println!(
            "    {}result{}: {}success{}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            husky_print_utils::GREEN,
            husky_print_utils::RESET,
        )
    }

    fn finalize_failure() {
        println!(
            "    {}result{}: {}failure{}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            husky_print_utils::RED,
            husky_print_utils::RESET,
        )
    }
}
