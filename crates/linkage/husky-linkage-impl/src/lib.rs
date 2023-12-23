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
    ($($linkage_impl: expr),*,) => {
        #[no_mangle]
        pub extern "C" fn linkage_impls(jar_index: __TaskJarIndex) -> AnyLinkageImpls {
            __set_jar_index(jar_index);
            let linkages: Vec<__LinkageImpl> =
                vec![
                    $($linkage_impl),*
                ];
            AnyLinkageImpls::new(linkages)
        }
    }
}

#[macro_export]
macro_rules! fn_linkage_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(ctx: __DevEvalContext, arguments: FnArguments) -> Value {
            __with_dev_eval_context(ctx, || {
                LinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
                    .fn_wrapper_aux(arguments);
                todo!();
            })
        }
        fn gn_wrapper(arguments: GnArguments) -> Value {
            todo!();
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coersion
        LinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
            .into_linkage_impl(fn_wrapper, gn_wrapper, $fn_item)
    }};
}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: expr) => {{
        fn gn_wrapper(arguments: GnArguments) -> Value {
            todo!();
        }
        todo!()
    }};
}

/// meant to be used in `LinkageImpl` definition
macro_rules! impl_into_linkage_impl {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, F, $($input,)* $output> IsLinkageImplSource<LinkageImpl<Pedestal>, fn($($input,)*) -> $output> for LinkageImplSource<LinkageImpl<Pedestal>, F>
        where
            Pedestal: Copy + 'static,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;
            type GnOutput = std::convert::Infallible;

            fn into_linkage_impl(
                self,
                fn_wrapper: fn(
                    DevEvalContext<LinkageImpl<Pedestal>>,
                    <LinkageImpl<Pedestal> as IsLinkageImpl>::FnArguments
                ) -> <LinkageImpl<Pedestal> as IsLinkageImpl>::Value,
                gn_wrapper: fn(<LinkageImpl<Pedestal> as IsLinkageImpl>::GnArguments)
                    -> <LinkageImpl<Pedestal> as IsLinkageImpl>::Value,
                m: fn($($input,)*) -> $output
            ) -> LinkageImpl<Pedestal> {
                LinkageImpl::RitchieFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(m)
                    },
                }
            }

            fn fn_wrapper_aux(self, arguments: <LinkageImpl<Pedestal> as IsLinkageImpl>::FnArguments)
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
