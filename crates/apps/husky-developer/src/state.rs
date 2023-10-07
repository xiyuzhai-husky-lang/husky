use husky_boot_task::BootTask;
use husky_devtime::{Devtime, IsDevtime};

#[derive(Default)]
pub struct DeveloperState {
    boot_devtime: Devtime<BootTask>,
    main_devtime: Option<Box<dyn IsDevtime>>,
}
