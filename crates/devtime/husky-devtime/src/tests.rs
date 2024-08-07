use crate::*;
use husky_path_utils::HuskyLangDevPaths;
use husky_standard_devsoul::StandardDevsoul;
use husky_standard_devsoul::StandardPedestal;
use husky_standard_visual_protocol::figure::StandardFigure;

// it looks ugly, lol
type StandardDevtime = Devtime<StandardDevsoul<StandardFigure<StandardPedestal>>>;

#[test]
fn devtime_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let devtime = StandardDevtime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
    let trace_bundles = devtime.trace_bundles();
}
