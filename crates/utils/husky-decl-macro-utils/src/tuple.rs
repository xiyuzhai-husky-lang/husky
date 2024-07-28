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
    (numbered $macro_name:ident) => {
        $macro_name!(1, T1);
        $macro_name!(2, T1, T2);
        $macro_name!(3, T1, T2, T3);
        $macro_name!(4, T1, T2, T3, T4);
        $macro_name!(5, T1, T2, T3, T4, T5);
        $macro_name!(6, T1, T2, T3, T4, T5, T6);
        $macro_name!(7, T1, T2, T3, T4, T5, T6, T7);
        $macro_name!(8, T1, T2, T3, T4, T5, T6, T7, T8);
        $macro_name!(9, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $macro_name!(10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $macro_name!(11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $macro_name!(12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    };
}
