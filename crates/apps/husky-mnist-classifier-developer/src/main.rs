use husky_dev_comptime::db::DevComptimeDb;
use husky_devtime::*;
use husky_graphics2d_visual_protocol::Graphics2dVisualProtocol;
use husky_ml_task::MlTask;
use std::path::PathBuf;

type Task = MlTask<DevComptimeDb, Graphics2dVisualProtocol>;

fn main() {
    let task = Task::new();
    let path: PathBuf = "examples/mnist-classifier".into();
    let devtime = Devtime::new(task, &path, None);
    devtime.serve_traces("localhost:51718")
}
