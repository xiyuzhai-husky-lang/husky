use husky_ad_hoc_protocol::AD_HOC_WEBSOCKET_ADDRESS;
use husky_devtime::*;
use husky_graphics2d_visual_protocol::figure::Graphics2dFigure;
use husky_ml_task::{MlPedestal, MlTask};
use std::path::PathBuf;

type Task = MlTask<Graphics2dFigure<MlPedestal>>;

fn main() {
    let task = Task::new();
    let path: PathBuf = "examples/mnist-classifier".into();
    let devtime = Devtime::new(task, &path, None).expect("valid");
    devtime.serve_traces(AD_HOC_WEBSOCKET_ADDRESS)
}
