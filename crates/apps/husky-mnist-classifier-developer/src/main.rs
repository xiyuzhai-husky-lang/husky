use husky_ad_hoc_protocol::AD_HOC_WEBSOCKET_ADDRESS;
use husky_devtime::*;
use husky_graphics2d_visual_protocol::figure::Graphics2dFigure;
use husky_standard_devsoul::{StandardDevsoul, StandardPedestal};
use std::path::PathBuf;

fn main() {
    let path: PathBuf = "examples/mnist-classifier".into();
    let devtime: Devtime<StandardDevsoul<Graphics2dFigure<StandardPedestal>>> =
        Devtime::new(&path, None).expect("valid");
    devtime.serve_traces(AD_HOC_WEBSOCKET_ADDRESS)
}
