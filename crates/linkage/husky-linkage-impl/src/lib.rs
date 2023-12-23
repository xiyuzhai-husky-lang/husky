#![feature(downcast_unchecked)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

use husky_task_prelude::{val_control_flow::ValControlFlow, LinkageImplValControlFlow};
use husky_task_prelude::{
    val_repr::{ValArgumentReprInterface, ValReprInterface},
    DevEvalContext, IsLinkageImpl,
};

pub trait IsFnLinkageImplSource<LinkageImpl: IsLinkageImpl, FnPointer> {
    type FnOutput;

    fn into_fn_linkage_impl(
        self,
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            arguments: &[ValArgumentReprInterface],
        ) -> LinkageImplValControlFlow<LinkageImpl>,
        fn_pointer: FnPointer,
    ) -> LinkageImpl;

    fn fn_wrapper_aux(
        self,
        ctx: DevEvalContext<LinkageImpl>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<LinkageImpl, Self::FnOutput>;
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
        fn fn_wrapper(
            ctx: __DevEvalContext,
            arguments: &[__ValArgumentReprInterface],
        ) -> __ValControlFlow {
            __with_dev_eval_context(ctx, || {
                FnLinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
                    .fn_wrapper_aux(ctx, arguments);
                todo!();
            })
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coersion
        FnLinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
            .into_fn_linkage_impl(fn_wrapper, $fn_item)
    }};
}

/// meant to be used in `LinkageImpl` definition
#[macro_export]
macro_rules! impl_is_fn_linkage_impl_source {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, F, $($input,)* $output> IsFnLinkageImplSource<LinkageImpl<Pedestal>, fn($($input,)*) -> $output> for FnLinkageImplSource<LinkageImpl<Pedestal>, F>
        where
            Pedestal: Copy + 'static,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linkage_impl(
                self,
                fn_wrapper: fn(
                    DevEvalContext<LinkageImpl<Pedestal>>,
                    &[ValArgumentReprInterface],
                ) -> ValControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinkageImpl<Pedestal> {
                LinkageImpl::RitchieFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn fn_wrapper_aux(
                self,
                ctx: DevEvalContext<LinkageImpl<Pedestal>>,
                arguments: &[ValArgumentReprInterface],
            ) -> ValControlFlow<Self::FnOutput> {
                let mut arguments = arguments.iter();
                ValControlFlow::Continue(self.1(
                    $(<$input as FromValue>::from_value(
                        ctx.eval_val_repr_argument(arguments.next().unwrap())?
                    ),)*
                ))
            }
        }
    };
}
