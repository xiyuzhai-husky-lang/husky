#[macro_export]
macro_rules! batch_into {
    ($e:expr) => {{
        $e.into_iter().map(|d| d.into()).collect()
    }};
}
