#[macro_export]
macro_rules! log_once {
    ($args: tt) => {
        unsafe {
            static mut printed: bool = false;
            if !printed {
                log::info!($args);
                printed = true
            }
        }
    };
}
