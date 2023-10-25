use std::sync::{Arc, Mutex};

use husky_boot_task::BootTask;
use husky_devtime::{Devtime, IsDevtime};

#[derive(Default)]
pub struct DeveloperState {
    boot_devtime: Devtime<BootTask>,
    main_devtime: Option<Arc<Mutex<dyn IsDevtime>>>,
}
