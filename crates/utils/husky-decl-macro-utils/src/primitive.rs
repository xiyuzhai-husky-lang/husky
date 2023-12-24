#[rustfmt::skip]
#[macro_export]
macro_rules! for_all_primitive_tys {
    ($macro_name:ident) => {
        $macro_name!(());
        $macro_name!(bool);
        $macro_name!(i8);
        $macro_name!(i16);
        $macro_name!(i32);
        $macro_name!(i64);
        $macro_name!(i128);
        $macro_name!(u8);
        $macro_name!(u16);
        $macro_name!(u32);
        $macro_name!(u64);
        $macro_name!(u128);
        $macro_name!(f32);
        $macro_name!(f64);
        $macro_name!(char);
    };
}
