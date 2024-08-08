use crate::*;
use husky_path_utils::HuskyLangDevPaths;
use husky_standard_devsoul::{StandardDevsoul, StandardPedestal};
use husky_standard_visual_protocol::figure::StandardFigure;

type StandardDevRuntime = DevRuntime<StandardDevsoul<StandardFigure<StandardPedestal>>>;

#[test]
fn dev_runtime_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let dev_runtime = StandardDevRuntime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
}

#[test]
fn dev_runtimes_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let dev_runtime = StandardDevRuntime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
    std::thread::spawn(move || {
        let dev_runtime = StandardDevRuntime::new(
            &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
            None,
        )
        .unwrap();
    })
    .join();
}
