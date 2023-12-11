pub use ad_hoc_task_dependency_macros::*;

#[macro_export]
macro_rules! init_crate {
    () => {};
}

#[macro_export]
macro_rules! linkage_impls {
    ($($linkage_impl: expr),*,) => {
        pub fn linkage_impls() -> Vec<::ad_hoc_task_dependency::LinkageImpl> {
            vec![
                $($linkage_impl.into_linkage_impl()),*
            ]
        }
    }
}

pub enum LinkageImpl {}

pub trait IntoLinkageImpl<M> {
    fn into_linkage_impl(self) -> LinkageImpl;
}

macro_rules! impl_into_linkage_impl {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, $($input,)* $output> IntoLinkageImpl<($($input,)* $output,)> for F
        where
            F: FnOnce($($input,)*) -> $output,
            $($input: Send, )*
            $output: Send,
        {
            fn into_linkage_impl(self) -> LinkageImpl {
                todo!()
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! all_ritchies {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}

all_ritchies! {impl_into_linkage_impl}
