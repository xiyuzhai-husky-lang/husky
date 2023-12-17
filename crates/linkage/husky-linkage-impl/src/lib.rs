#![feature(downcast_unchecked)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

pub trait IsLinkageImplSource<LinkageImpl: IsLinkageImpl, Marker> {
    type FnOutput;
    type GnOutput;

    fn into_linkage_impl(
        self,
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl>,
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
        pub extern "C" fn linkage_impls(jar_index: __TaskJarIndex) -> AnyLinkageImpls {
            __set_jar_index(jar_index);
            let linkages: Vec<__LinkageImpl> =
                vec![
                    $({
                        fn fn_wrapper(ctx: __DevEvalContext, arguments: FnArguments) -> Value {
                            __with_dev_eval_context(
                                ctx,
                                || {
                                    LinkageImplSource(
                                        std::marker::PhantomData::<__LinkageImpl>,
                                        $linkage_impl_src,
                                    ).fn_wrapper_aux(arguments);
                                    todo!();
                                }
                            )
                        }
                        fn gn_wrapper(arguments: GnArguments) -> Value {
                            todo!();
                        }
                        // pass `linkage_impl_src` two times
                        // - one time is to determine the parameter types and return type
                        // - the other time is to actually give the fn pointer with implicit coersion
                        LinkageImplSource(
                            std::marker::PhantomData::<__LinkageImpl>,
                            $linkage_impl_src,
                        )
                            .into_linkage_impl(fn_wrapper, gn_wrapper, $linkage_impl_src)}),*
                ];
            AnyLinkageImpls::new(linkages)
        }
    }
}

/// meant to be used in `LinkageImpl` definition
macro_rules! impl_into_linkage_impl {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<BasePoint, F, $($input,)* $output> IsLinkageImplSource<LinkageImpl<BasePoint>, fn($($input,)*) -> $output> for LinkageImplSource<LinkageImpl<BasePoint>, F>
        where
            BasePoint: Copy + 'static,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;
            type GnOutput = std::convert::Infallible;

            fn into_linkage_impl(
                self,
                fn_wrapper: fn(
                    DevEvalContext<LinkageImpl<BasePoint>>,
                    <LinkageImpl<BasePoint> as IsLinkageImpl>::FnArguments
                ) -> <LinkageImpl<BasePoint> as IsLinkageImpl>::Value,
                gn_wrapper: fn(<LinkageImpl<BasePoint> as IsLinkageImpl>::GnArguments)
                    -> <LinkageImpl<BasePoint> as IsLinkageImpl>::Value,
                m: fn($($input,)*) -> $output
            ) -> LinkageImpl<BasePoint> {
                LinkageImpl::RitchieFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(m)
                    },
                }
            }

            fn fn_wrapper_aux(self, arguments: <LinkageImpl<BasePoint> as IsLinkageImpl>::FnArguments)
                -> Self::FnOutput {
                let mut arguments = arguments.into_iter();
                self.1(
                    $(<$input as FromValue>::from_value(
                        arguments.next().unwrap()
                    ),)*
                )
            }
        }
    };
}
use husky_task_prelude::{DevEvalContext, IsLinkageImpl};
pub(crate) use impl_into_linkage_impl;

use smallvec::SmallVec;
