use husky_boot_task::BootTask;
use husky_devtime::{Devtime, IsDevtime};

pub struct DeveloperState {
    boot_devtime: Devtime<BootTask>,
    devtime: Box<dyn IsDevtime>,
}
