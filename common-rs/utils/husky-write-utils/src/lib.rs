#[macro_export]
macro_rules! w {
    ($formatter: expr, $($args: tt),*) => {{
        write!($formatter, $($args),*).unwrap()
    }};
    ($formatter: expr; $formattee: expr) => {{
        write!($formatter, "{}", $formattee).unwrap()
    }};
}
