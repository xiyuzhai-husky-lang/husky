#[macro_export]
macro_rules! insert_new {
    ($map:expr, $key: expr, $value: expr) => {
        should!($map.insert($key, $value).is_none());
    };
}
