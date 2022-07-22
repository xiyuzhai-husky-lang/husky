pub use futures::executor::block_on as __block_on;
pub use husky_trace_protocol::SampleId as __SampleId;
pub use test_utils::TestResult as __TestResult;

use print_utils::*;

pub fn __serve_on_error_init() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            let trimmed_file = trim(location.file());
            println!(
                "{RED}panic at '{}'{RESET}, {GREEN}{}/{}:{YELLOW}{}:{}{RESET}",
                if let Some(message) = panic_info.message() {
                    format!("{}", message)
                } else if let Some(payload) = panic_info.payload().downcast_ref::<&'static str>() {
                    format!("'{}', ", payload)
                } else {
                    "".to_owned()
                },
                std::env::var("CARGO_MANIFEST_DIR").unwrap(),
                trimmed_file,
                location.line(),
                location.column(),
            );
        } else {
            println!("panic at '' but can't get location information...");
        }
    }));
}

fn trim(path: &str) -> &str {
    path.split_at(path.find("src/").unwrap()).1
}
