#[macro_export]
macro_rules! loop_require {
    ($condition: expr) => {
        if !$condition {
            continue;
        }
    };
}
