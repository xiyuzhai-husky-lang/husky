#[macro_export]
macro_rules! w {
    ($formatter: expr; $formattee: expr) => {{
        write!($formatter, "{}", $formattee)?
    }};
}
