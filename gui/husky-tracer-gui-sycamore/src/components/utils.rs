macro_rules! class {
    (*$var: ident) => {{
        if $var.cget() {
            stringify!($var)
        } else {
            ""
        }
    }};
    ($var: ident) => {{
        if $var {
            stringify!($var)
        } else {
            ""
        }
    }};
}
pub(super) use class;
