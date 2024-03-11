#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

use husky_task_interface::{ki_control_flow::KiControlFlow, LinkageImplKiControlFlow};
use husky_task_interface::{
    ki_repr::{KiArgumentReprInterface, KiReprInterface},
    DevEvalContext, IsLinkageImpl,
};

pub trait IsFnLinkageImplSource<LinkageImpl: IsLinkageImpl, FnPointer> {
    type FnOutput;

    fn into_fn_linkage_impl(
        self,
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        fn_pointer: FnPointer,
    ) -> LinkageImpl;

    fn fn_wrapper_aux(
        self,
        ctx: DevEvalContext<LinkageImpl>,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! linkage_impls {
    ($($linkage_impl: expr),* $(,)?) => {
        #[no_mangle]
        pub extern "C" fn linkage_impls(jar_index: __TaskJarIndex) -> AnyLinkageImpls {
            __set_jar_index(jar_index);
            let linkages: Vec<__LinkageImpl> =
                vec![
                    $($linkage_impl),*
                ];
            AnyLinkageImpls::new(linkages)
        }
    };
}

#[macro_export]
macro_rules! fn_linkage_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(
            ctx: __DevEvalContext,
            arguments: &[__KiArgumentReprInterface],
        ) -> __KiControlFlow {
            __with_dev_eval_context(ctx, || {
                // todo: catch unwind
                __KiControlFlow::Continue(
                    __ValueLeashTest(
                        FnLinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
                            .fn_wrapper_aux(ctx, arguments)?,
                    )
                    .into_value(),
                )
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
            Pedestal: std::fmt::Debug + Copy + 'static,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linkage_impl(
                self,
                fn_wrapper: fn(
                    DevEvalContext<LinkageImpl<Pedestal>>,
                    &[KiArgumentReprInterface],
                ) -> StandardLinkageImplKiControlFlow,
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
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinkageImplKiControlFlow<Self::FnOutput> {
                let mut arguments = arguments.iter();
                let value_stands = &mut ValueStands::default();
                KiControlFlow::Continue(self.1(
                    $({
                        let argument = arguments.next().unwrap();
                        match *argument {
                            KiArgumentReprInterface::Simple(ki_repr_interface) => {
                                <$input as FromValue>::from_value_temp(
                                    ctx.eval_ki_repr_interface(ki_repr_interface)?,
                                    (value_stands)
                                )
                            },
                            KiArgumentReprInterface::Keyed(argument) => todo!("KiArgumentReprInterface::Keyed(argument)"),
                            KiArgumentReprInterface::Variadic(ref ki_repr_interfaces) => {
                                <$input as FromValue>::from_variadic_values(
                                    ki_repr_interfaces.iter().map(
                                        |&ki_repr_interface| ctx.eval_ki_repr_interface(ki_repr_interface)
                                    ),
                                    Some(value_stands)
                                )?
                            },
                            KiArgumentReprInterface::Branch { .. } => unreachable!(),
                            KiArgumentReprInterface::RuntimeConstants(ref argument) => todo!(),
                        }},)*
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! ty_default_linkage_impl {
    ($ty: ty) => {
        fn_linkage_impl!(|| <$ty as Default>::default())
    };
}

// unveils

pub trait IsUnveilFnLinkageImplSource<LinkageImpl: IsLinkageImpl, Target, FnPointer> {
    type FnOutput;

    fn into_unveil_linkage_impl(
        self,
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            arguments: &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        fn_pointer: FnPointer,
    ) -> LinkageImpl;

    fn unveil_fn_wrapper_aux(
        self,
        ctx: DevEvalContext<LinkageImpl>,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! unveil_linkage_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(
            ctx: __DevEvalContext,
            arguments: &[__KiArgumentReprInterface],
        ) -> __KiControlFlow {
            __with_dev_eval_context(ctx, || {
                // todo: catch unwind
                __KiControlFlow::Continue(
                    __ValueLeashTest(
                        UnveilFnLinkageImplSource(
                            std::marker::PhantomData::<__LinkageImpl>,
                            $fn_item,
                        )
                        .unveil_fn_wrapper_aux(ctx, arguments)?,
                    )
                    .into_value(),
                )
            })
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coersion
        UnveilFnLinkageImplSource(std::marker::PhantomData::<__LinkageImpl>, $fn_item)
            .into_unveil_linkage_impl(fn_wrapper, $fn_item)
    }};
}

/// meant to be used in `LinkageImpl` definition
#[macro_export]
macro_rules! impl_is_unveil_linkage_impl_source {
    (
        [$($runtime_constant: ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, F, B, Target, $($runtime_constant,)* $output> IsUnveilFnLinkageImplSource<
            LinkageImpl<Pedestal>,
            Target,
            fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>
        > for UnveilFnLinkageImplSource<LinkageImpl<Pedestal>, F>
        where
            Pedestal: std::fmt::Debug + Copy + 'static,
            F: Fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            B: IntoValue, // no need to use ValueLeashTest because B is definitely not leashed
            Target: Send + FromValue,
            $($runtime_constant: Send + FromValue,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_unveil_linkage_impl(
                self,
                fn_wrapper: fn(
                    DevEvalContext<LinkageImpl<Pedestal>>,
                    &[KiArgumentReprInterface],
                ) -> StandardLinkageImplKiControlFlow,
                fn_pointer: fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            ) -> LinkageImpl<Pedestal> {
                LinkageImpl::RitchieUnveilFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn unveil_fn_wrapper_aux(
                self,
                ctx: DevEvalContext<LinkageImpl<Pedestal>>,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinkageImplKiControlFlow<Self::FnOutput> {
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentReprInterface::Simple(target) = arguments[0] else {
                    unreachable!("expect ordinary argument")
                };
                let KiArgumentReprInterface::RuntimeConstants(
                    ref runtime_constants
                ) = arguments[1] else {
                    unreachable!("expect runtime constants, but got {:?} instead", arguments[1])
                };
                let value_stands = &mut ValueStands::default();
                let mut runtime_constants = runtime_constants.iter();
                match self.1(
                    <Target as FromValue>::from_value_temp(
                        ctx.eval_ki_repr_interface(target)?,
                        value_stands
                    ),
                    ($(<$runtime_constant as FromValue>::from_value_temp(
                        ctx.eval_val_runtime_constant(
                            *runtime_constants.next().expect("missing runtime constant")
                        ),
                        value_stands
                    ),)*)
                ) {
                    std::ops::ControlFlow::Continue(c) => KiControlFlow::Continue(c),
                    std::ops::ControlFlow::Break(b) => KiControlFlow::Return(b.into_value()),
                }
            }
        }
    };
}
