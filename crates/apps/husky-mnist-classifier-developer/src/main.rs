use husky_dev_comptime::db::DevComptimeDb;
use husky_devtime::*;
use husky_graphics2d_visual_protocol::Graphics2dVisualProtocol;
use husky_ml_task::MlTask;

type Task = MlTask<DevComptimeDb, Graphics2dVisualProtocol>;

fn main() {
    let task = Task::new();
    println!("Hello, world!");
}
