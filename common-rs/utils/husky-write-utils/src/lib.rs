#[macro_export]
macro_rules! w {
    ($formatter: expr, $($args: tt),*) => {{
        write!($formatter, $($args),*)?
    }};
    ($formatter: expr; $formattee: expr) => {{
        write!($formatter, "{}", $formattee)?
    }};
}
