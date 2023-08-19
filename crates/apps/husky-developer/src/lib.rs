// #![feature(try_trait_v2)]
// mod config;
// mod error;
// mod gui;
// mod instance;
// mod internal;
// mod notif;
// mod ops;

// pub use config::HuskyDebuggerConfig;
// pub use error::{DebuggerError, DebuggerResult};

// use futures::executor::ThreadPool;
// use gui::handle_query;
// use husky_debugtime::Devtime;
// use husky_path_utils::collect_package_dirs;
// use husky_print_utils::*;

// use husky_test_utils::TestResult;
// use husky_trace_protocol::*;

// use instance::*;
// use internal::HuskyDebuggerInternal;
// use notif::handle_notif;
// use ops::*;
// use std::{
//     convert::Infallible,
//     net::ToSocketAddrs,
//     path::{Path, PathBuf},
//     sync::Arc,
// };
// use std::{sync::Mutex, time::Instant};
// use warp::Filter;

// pub async fn dev_run(package_dir: PathBuf) -> DebuggerResult<()> {
//     let husky_devserver = HuskyDebuggerInstance::new(HuskyDebuggerConfig {
//         package_dir,
//         opt_sample_id: None,
//         verbose: false,
//         compiled: false,
//     });
//     husky_devserver.serve("localhost:51617").await
// }

// pub async fn dev_test(packages_dir: PathBuf) {
//     assert!(packages_dir.is_dir());
//     let package_dirs = collect_package_dirs(&packages_dir);
//     println!(
//         "\n{}Running{} tests on {} example packages:",
//         husky_print_utils::CYAN,
//         husky_print_utils::RESET,
//         package_dirs.len()
//     );

//     for package_dir in package_dirs {
//         println!(
//             "\n{}test{} {}",
//             husky_print_utils::CYAN,
//             husky_print_utils::RESET,
//             package_dir.as_os_str().to_str().unwrap(),
//         );
//         let husky_devserver = HuskyDebuggerInstance::new(HuskyDebuggerConfig {
//             package_dir,
//             opt_sample_id: Some(SampleId(23)),
//             verbose: false,
//             compiled: false,
//         });
//         finalize(
//             husky_devserver
//                 .serve_on_error("localhost:51617", SampleId(0))
//                 .await,
//         )
//     }
// }

// fn finalize(test_result: TestResult) {
//     match test_result {
//         TestResult:: Ok => finalize_success(),
//         TestResult::Err => finalize_failure(),
//     }
// }

// fn finalize_success() {
//     println!(
//         "    {}result{}: {}success{}",
//         husky_print_utils::CYAN,
//         husky_print_utils::RESET,
//         husky_print_utils::GREEN,
//         husky_print_utils::RESET,
//     )
// }

// fn finalize_failure() {
//     println!(
//         "    {}result{}: {}failure{}",
//         husky_print_utils::CYAN,
//         husky_print_utils::RESET,
//         husky_print_utils::RED,
//         husky_print_utils::RESET,
//     )
// }
