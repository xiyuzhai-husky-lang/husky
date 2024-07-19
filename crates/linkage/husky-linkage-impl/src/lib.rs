#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinkageImpls;

use husky_devsoul_interface::{
    devsoul::IsDevsoulInterface,
    ki_control_flow::KiControlFlow,
    ki_repr::{KiArgumentReprInterface, KiReprInterface},
    DevEvalContext, IsLinkageImpl, LinkageImplKiControlFlow,
};

pub trait IsFnLinkageImplSource<LinkageImpl: IsLinkageImpl, FnPointer> {
    type FnOutput;

    fn into_fn_linkage_impl(
        self,
        fn_wrapper: fn(&[KiArgumentReprInterface]) -> LinkageImplKiControlFlow<LinkageImpl>,
        fn_pointer: FnPointer,
    ) -> LinkageImpl;

    fn fn_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl, Self::FnOutput>;
}

/// generates the function to acquire linkage impls accessed through dynamic library,
///
/// it also set up the jar index.
#[macro_export]
macro_rules! linkage_impls {
    ($($linkage_impl: expr),* $(,)?) => {
        #[no_mangle]
        pub extern "C" fn linkage_impls(jar_index: __HuskyJarIndex) -> AnyLinkageImpls {
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
        fn fn_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                __ValueLeashTest(
                    FnLinkageImplSource::<__Pedestal, __DevsoulInterface, _>(
                        std::marker::PhantomData,
                        $fn_item,
                    )
                    .fn_wrapper_aux(arguments)?,
                )
                .into_value(),
            )
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coercion
        FnLinkageImplSource::<__Pedestal, __DevsoulInterface, _>(std::marker::PhantomData, $fn_item)
            .into_fn_linkage_impl(fn_wrapper, $fn_item)
    }};
}

#[test]
fn fn_linkage_impl_works() {
    use crate::standard::{ugly::*, *};
    use crate::IsFnLinkageImplSource;
    use husky_devsoul_interface::ugly::*;
    use husky_standard_devsoul_interface::ugly::*;

    type __LinkageImpl = StandardLinkageImpl<__Pedestal>;
    type __DevEvalContext = DevEvalContext<__LinkageImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinkageImpl = __LinkageImpl;

        fn eval_context() -> DevEvalContext<Self::LinkageImpl> {
            todo!()
        }
    }

    fn_linkage_impl!(|| ());
}

/// meant to be used in `LinkageImpl` definition
#[macro_export]
macro_rules! impl_is_fn_linkage_impl_source {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, $($input,)* $output> IsFnLinkageImplSource<
            LinkageImpl<Pedestal>,
            fn($($input,)*) -> $output
        > for FnLinkageImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<
                LinkageImpl = LinkageImpl<Pedestal>
            >,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linkage_impl(
                self,
                fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinkageImplKiControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinkageImpl<Pedestal> {
                LinkageImpl::RitchieFn {
                    fn_ki_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn fn_wrapper_aux(
                self,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinkageImplKiControlFlow<Self::FnOutput> {
                let ctx = DevsoulInterface::eval_context();
                #[allow(unused_variables)]
                let mut arguments = arguments.iter();
                #[allow(unused_variables)]
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
            arguments: &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        fn_pointer: FnPointer,
    ) -> LinkageImpl;

    fn unveil_fn_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! unveil_fn_linkage_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                __ValueLeashTest(
                    UnveilFnLinkageImplSource::<__Pedestal, __DevsoulInterface, _>(
                        std::marker::PhantomData,
                        $fn_item,
                    )
                    .unveil_fn_wrapper_aux(arguments)?,
                )
                .into_value(),
            )
        }
        // pass `$fn_item` two times
        // - one time is to determine the parameter types and return type
        // - the other time is to actually give the fn pointer with implicit coercion
        UnveilFnLinkageImplSource::<__Pedestal, __DevsoulInterface, _>(
            std::marker::PhantomData,
            $fn_item,
        )
        .into_unveil_linkage_impl(fn_wrapper, $fn_item)
    }};
}

#[test]
fn unveil_fn_linkage_impl_works() {
    use crate::{
        standard::{ugly::*, *},
        IsFnLinkageImplSource, IsUnveilFnLinkageImplSource,
    };
    use husky_devsoul_interface::ugly::*;
    use husky_standard_devsoul_interface::ugly::*;

    type __LinkageImpl = StandardLinkageImpl<__Pedestal>;
    type __DevEvalContext = DevEvalContext<__LinkageImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinkageImpl = __LinkageImpl;

        fn eval_context() -> DevEvalContext<Self::LinkageImpl> {
            todo!()
        }
    }

    unveil_fn_linkage_impl!(|_: i32, ()| -> std::ops::ControlFlow<i32, i32> {
        std::ops::ControlFlow::Continue(0)
    });
}

/// meant to be used in `LinkageImpl` definition
#[macro_export]
macro_rules! impl_is_unveil_fn_linkage_impl_source {
    (
        [$($runtime_constant: ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, B, Target, $($runtime_constant,)* $output> IsUnveilFnLinkageImplSource<
            LinkageImpl<Pedestal>,
            Target,
            fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>
        > for UnveilFnLinkageImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<LinkageImpl = LinkageImpl<Pedestal>>,
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
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinkageImplKiControlFlow<Self::FnOutput> {
                let ctx = DevsoulInterface::eval_context();
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
