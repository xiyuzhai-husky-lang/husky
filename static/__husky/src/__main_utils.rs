pub use futures::executor::block_on as __block_on;
pub use husky_devserver::{HuskyDebugger, HuskyDebuggerConfig};
pub use husky_test_utils::TestResult as __TestResult;
pub use husky_trace_protocol_old::SampleId as __SampleId;

use husky_print_utils::*;

pub fn __serve_on_error_init() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let adjusted_path = adjust(&cargo_manifest_dir, location.file());
            println!(
                "{RED}panic at '{}'{RESET}, {GREEN}{}:{YELLOW}{}:{}{RESET}",
                if let Some(message) = panic_info.message() {
                    format!("{}", message)
                } else if let Some(payload) = panic_info.payload().downcast_ref::<&'static str>() {
                    format!("'{}', ", payload)
                } else {
                    "".to_owned()
                },
                adjusted_path,
                location.line(),
                location.column(),
            );
        } else {
            println!("panic at '' but can't get location information...");
        }
    }));
}

fn adjust<'a>(prefix: &'a str, path: &'a str) -> String {
    let mid = path.find("src/").unwrap();
    let path_split = path.split_at(mid);
    if prefix.split_at(prefix.len() - mid).1 == path_split.0 {
        format!("{}/{}", prefix, path.split_at(mid).1)
    } else {
        path.to_string()
    }
}
