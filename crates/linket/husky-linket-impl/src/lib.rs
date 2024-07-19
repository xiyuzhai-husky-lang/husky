#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]
pub mod any;
#[cfg(feature = "standard")]
pub mod standard;

pub use self::any::AnyLinketImpls;

use husky_devsoul_interface::{
    devsoul::IsDevsoulInterface,
    ki_control_flow::KiControlFlow,
    ki_repr::{KiArgumentReprInterface, KiReprInterface},
    DevEvalContext, IsLinketImpl, LinketImplKiControlFlow,
};

pub trait IsFnLinketImplSource<LinketImpl: IsLinketImpl, FnPointer> {
    type FnOutput;

    fn into_fn_linket_impl(
        self,
        fn_wrapper: fn(&[KiArgumentReprInterface]) -> LinketImplKiControlFlow<LinketImpl>,
        fn_pointer: FnPointer,
    ) -> LinketImpl;

    fn fn_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<LinketImpl, Self::FnOutput>;
}

/// generates the function to acquire linket impls accessed through dynamic library,
///
/// it also set up the jar index.
#[macro_export]
macro_rules! linket_impls {
    ($($linket_impl: expr),* $(,)?) => {
        #[no_mangle]
        pub extern "C" fn linket_impls(jar_index: __HuskyJarIndex) -> AnyLinketImpls {
            __set_jar_index(jar_index);
            let linkets: Vec<__LinketImpl> =
                vec![
                    $($linket_impl),*
                ];
            AnyLinketImpls::new(linkets)
        }
    };
}

#[macro_export]
macro_rules! fn_linket_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                __ValueLeashTest(
                    FnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(
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
        FnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(std::marker::PhantomData, $fn_item)
            .into_fn_linket_impl(fn_wrapper, $fn_item)
    }};
}

#[test]
fn fn_linket_impl_works() {
    use crate::standard::{ugly::*, *};
    use crate::IsFnLinketImplSource;
    use husky_devsoul_interface::ugly::*;
    use husky_standard_devsoul_interface::ugly::*;

    type __LinketImpl = StandardLinketImpl<__Pedestal>;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinketImpl = __LinketImpl;

        fn eval_context() -> DevEvalContext<Self::LinketImpl> {
            todo!()
        }
    }

    fn_linket_impl!(|| ());
}

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_fn_linket_impl_source {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, $($input,)* $output> IsFnLinketImplSource<
            LinketImpl<Pedestal>,
            fn($($input,)*) -> $output
        > for FnLinketImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<
                LinketImpl = LinketImpl<Pedestal>
            >,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue, )*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linket_impl(
                self,
                fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinketImplKiControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinketImpl<Pedestal> {
                LinketImpl::RitchieFn {
                    fn_ki_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn fn_wrapper_aux(
                self,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinketImplKiControlFlow<Self::FnOutput> {
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
macro_rules! ty_default_linket_impl {
    ($ty: ty) => {
        fn_linket_impl!(|| <$ty as Default>::default())
    };
}

// unveils

pub trait IsUnveilFnLinketImplSource<LinketImpl: IsLinketImpl, Target, FnPointer> {
    type FnOutput;

    fn into_unveil_linket_impl(
        self,
        fn_wrapper: fn(
            arguments: &[KiArgumentReprInterface],
        ) -> LinketImplKiControlFlow<LinketImpl>,
        fn_pointer: FnPointer,
    ) -> LinketImpl;

    fn unveil_fn_wrapper_aux(
        self,
        arguments: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<LinketImpl, Self::FnOutput>;
}

#[macro_export]
macro_rules! unveil_fn_linket_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                __ValueLeashTest(
                    UnveilFnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(
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
        UnveilFnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(
            std::marker::PhantomData,
            $fn_item,
        )
        .into_unveil_linket_impl(fn_wrapper, $fn_item)
    }};
}

#[test]
fn unveil_fn_linket_impl_works() {
    use crate::{
        standard::{ugly::*, *},
        IsFnLinketImplSource, IsUnveilFnLinketImplSource,
    };
    use husky_devsoul_interface::ugly::*;
    use husky_standard_devsoul_interface::ugly::*;

    type __LinketImpl = StandardLinketImpl<__Pedestal>;
    type __DevEvalContext = DevEvalContext<__LinketImpl>;
    struct __DevsoulInterface;
    impl IsDevsoulInterface for __DevsoulInterface {
        type LinketImpl = __LinketImpl;

        fn eval_context() -> DevEvalContext<Self::LinketImpl> {
            todo!()
        }
    }

    unveil_fn_linket_impl!(|_: i32, ()| -> std::ops::ControlFlow<i32, i32> {
        std::ops::ControlFlow::Continue(0)
    });
}

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_unveil_fn_linket_impl_source {
    (
        [$($runtime_constant: ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, B, Target, $($runtime_constant,)* $output> IsUnveilFnLinketImplSource<
            LinketImpl<Pedestal>,
            Target,
            fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>
        > for UnveilFnLinketImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<LinketImpl = LinketImpl<Pedestal>>,
            F: Fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            B: IntoValue, // no need to use ValueLeashTest because B is definitely not leashed
            Target: Send + FromValue,
            $($runtime_constant: Send + FromValue,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_unveil_linket_impl(
                self,
                fn_wrapper: fn(
                    &[KiArgumentReprInterface],
                ) -> StandardLinketImplKiControlFlow,
                fn_pointer: fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            ) -> LinketImpl<Pedestal> {
                LinketImpl::RitchieUnveilFn {
                    fn_wrapper,
                    fn_pointer: unsafe {
                        std::mem::transmute(fn_pointer)
                    },
                }
            }

            fn unveil_fn_wrapper_aux(
                self,
                arguments: &[KiArgumentReprInterface],
            ) -> StandardLinketImplKiControlFlow<Self::FnOutput> {
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
