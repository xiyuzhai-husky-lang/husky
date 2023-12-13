pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}

pub trait IntoLinkageImpl<LinkageImpl, Marker> {
    fn into_linkage_impl(self) -> LinkageImpl;
}

#[macro_export]
macro_rules! linkage_impls {
    ($($linkage_impl: expr),*,) => {
        #[no_mangle]
        pub extern "C" fn linkage_impls() -> AnyLinkageImpls {
            AnyLinkageImpls::new(vec![
                $(LinkageImplSource($linkage_impl).into_linkage_impl()),*
            ])
        }
    }
}

macro_rules! impl_into_linkage_impl {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, $($input,)* $output> IntoLinkageImpl<LinkageImpl, ($($input,)* $output,)> for LinkageImplSource<F>
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
pub(crate) use impl_into_linkage_impl;

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
pub(crate) use all_ritchies;
