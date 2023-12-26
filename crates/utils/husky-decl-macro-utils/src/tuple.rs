#[rustfmt::skip]
#[macro_export]
macro_rules! for_all_non_unit_tuple_tys {
    ($macro_name:ident) => {
        $macro_name!(T1);
        $macro_name!(T1, T2);
        $macro_name!(T1, T2, T3);
        $macro_name!(T1, T2, T3, T4);
        $macro_name!(T1, T2, T3, T4, T5);
        $macro_name!(T1, T2, T3, T4, T5, T6);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7, T8);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $macro_name!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    };
}
