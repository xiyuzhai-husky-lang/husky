#![feature(downcast_unchecked)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Value;
    type FnArguments: Default;
    type GnArguments;

    fn eval_fn(self, arguments: Self::FnArguments) -> Self::Value;
    fn eval_gn(self) -> Self::Value;
}

pub trait IsLinkageImplSource<LinkageImpl: IsLinkageImpl, Marker> {
    type FnOutput;
    type GnOutput;

    fn into_linkage_impl(
        self,
        fn_wrapper: fn(
            <LinkageImpl as IsLinkageImpl>::FnArguments,
        ) -> <LinkageImpl as IsLinkageImpl>::Value,
        gn_wrapper: fn(
            <LinkageImpl as IsLinkageImpl>::GnArguments,
        ) -> <LinkageImpl as IsLinkageImpl>::Value,
        m: Marker,
    ) -> LinkageImpl;

    fn fn_wrapper_aux(
        self,
        arguments: <LinkageImpl as IsLinkageImpl>::FnArguments,
    ) -> Self::FnOutput;
}

#[macro_export]
macro_rules! linkage_impls {
    ($($linkage_impl_src: expr),*,) => {
        #[no_mangle]
        pub extern "C" fn linkage_impls() -> AnyLinkageImpls {
            AnyLinkageImpls::new(vec![
                $({
                    fn fn_wrapper(arguments: FnArguments) -> Value {
                        LinkageImplSource($linkage_impl_src).fn_wrapper_aux(arguments);
                        todo!();
                    }
                    fn gn_wrapper(arguments: GnArguments) -> Value {
                        todo!();
                    }
                    // pass `linkage_impl_src` two times
                    // - one time is to determine the parameter types and return type
                    // - the other time is to actually give the fn pointer with implicit coersion
                    LinkageImplSource($linkage_impl_src)
                        .into_linkage_impl(fn_wrapper, gn_wrapper, $linkage_impl_src)}),*
            ])
        }
    }
}

/// meant to be used in `LinkageImpl` definition
macro_rules! impl_into_linkage_impl {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, $($input,)* $output> IsLinkageImplSource<LinkageImpl, fn($($input,)*) -> $output> for LinkageImplSource<F>
        where
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;
            type GnOutput = std::convert::Infallible;

            fn into_linkage_impl(
                self,
                fn_wrapper: fn(<LinkageImpl as IsLinkageImpl>::FnArguments)
                    -> <LinkageImpl as IsLinkageImpl>::Value,
                gn_wrapper: fn(<LinkageImpl as IsLinkageImpl>::GnArguments)
                    -> <LinkageImpl as IsLinkageImpl>::Value,
                m: fn($($input,)*) -> $output
            ) -> LinkageImpl {
                LinkageImpl::RitchieFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(m)
                    },
                }
            }

            fn fn_wrapper_aux(self, arguments: <LinkageImpl as IsLinkageImpl>::FnArguments)
                -> Self::FnOutput {
                let mut arguments = arguments.into_iter();
                self.0(
                    $(<$input as FromValue>::from_value(
                        arguments.next().unwrap()
                    ),)*
                )
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
use smallvec::SmallVec;
