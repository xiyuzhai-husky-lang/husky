use husky_boot_task::BootTask;
use husky_devtime::{Devtime, IsDevtime};

pub struct DeveloperState {
    boot_devtime: Devtime<BootTask>,
    main_devtime: Option<Box<dyn IsDevtime>>,
}
