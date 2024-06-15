pub type Seq<T> = &'static [T];

#[macro_use]
macro_rules! ret {
    ($expr: expr) => {
        return expr
    };
}

pub(crate) use ret;
