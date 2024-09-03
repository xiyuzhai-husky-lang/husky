#![feature(downcast_unchecked)]
#![feature(trait_upcasting)]
pub mod eval_context;
pub mod exception;
pub mod linket_impl;
pub mod linket_impls;
pub mod pedestal;
pub mod static_var;
#[cfg(feature = "ugly")]
pub mod ugly;
pub mod var_id;

use crate::pedestal::IsPedestalFull;
use eval_context::{DevEvalContext, IsDevRuntimeDyn};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::KiArgumentReprInterface;
use husky_value::vm_control_flow::VmControlFlow;
use husky_wild_utils::arb_ref;
use linket_impl::{
    IsLinketImpl, LinketImplFrozenValue, LinketImplKiControlFlow, LinketImplThawedValue,
};

pub type LinketImplVmControlFlowThawed<LinketImpl, C = LinketImplThawedValue<LinketImpl>> =
    VmControlFlow<C, LinketImplThawedValue<LinketImpl>, <LinketImpl as IsLinketImpl>::Exception>;
pub type LinketImplVmControlFlowFrozen<LinketImpl, C = LinketImplFrozenValue<LinketImpl>> =
    VmControlFlow<C, LinketImplFrozenValue<LinketImpl>, <LinketImpl as IsLinketImpl>::Exception>;

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

#[macro_export]
macro_rules! fn_linket_impl {
    ($fn_item: expr) => {{
        fn fn_wrapper(arguments: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            // todo: catch unwind
            __KiControlFlow::Continue(
                FnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(
                    std::marker::PhantomData,
                    $fn_item,
                )
                .fn_wrapper_aux(arguments)?
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

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_fn_linket_impl_source {
    (
        [$($input:ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, $($input,)* $output> IsFnLinketImplSource<
            LinketImpl,
            fn($($input,)*) -> $output
        > for FnLinketImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<
                LinketImpl = LinketImpl
            >,
            F: Fn($($input,)*) -> $output,
            $($input: Send + FromValue,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_fn_linket_impl(
                self,
                fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardKiControlFlow,
                fn_pointer: fn($($input,)*) -> $output
            ) -> LinketImpl {
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
            ) -> StandardKiControlFlow<Self::FnOutput> {
                let ctx = DevsoulInterface::dev_eval_context();
                #[allow(unused_variables)]
                let mut arguments = arguments.iter();
                #[allow(unused_variables)]
                let slush_values = &mut SlushValues::default();
                ki_catch_unwind!(
                    self.1,
                    $({
                        let argument = arguments.next().unwrap();
                        match *argument {
                            KiArgumentReprInterface::Simple(ki_repr_interface) => {
                                <$input as FromValue>::from_value_temp(
                                    ctx.eval_ki_repr_interface(ki_repr_interface)?,
                                    (slush_values)
                                )
                            },
                            KiArgumentReprInterface::Keyed(argument) => todo!("KiArgumentReprInterface::Keyed(argument)"),
                            KiArgumentReprInterface::Variadic(ref ki_repr_interfaces) => {
                                <$input as FromValue>::from_variadic_values(
                                    ki_repr_interfaces.iter().map(
                                        |&ki_repr_interface| ctx.eval_ki_repr_interface(ki_repr_interface)
                                    ),
                                    Some(slush_values)
                                )?
                            },
                            KiArgumentReprInterface::Branch { .. } => unreachable!(),
                            KiArgumentReprInterface::RuntimeConstants(ref argument) => todo!(),
                        }}),*
                )
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
                UnveilFnLinketImplSource::<__Pedestal, __DevsoulInterface, _>(
                    std::marker::PhantomData,
                    $fn_item,
                )
                .unveil_fn_wrapper_aux(arguments)?
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

/// meant to be used in `LinketImpl` definition
#[macro_export]
macro_rules! impl_is_unveil_fn_linket_impl_source {
    (
        [$($runtime_constant: ident),*], $output:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<Pedestal, DevsoulInterface, F, B, Target, $($runtime_constant,)* $output> IsUnveilFnLinketImplSource<
            LinketImpl,
            Target,
            fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>
        > for UnveilFnLinketImplSource<Pedestal, DevsoulInterface, F>
        where
            Pedestal: IsPedestalFull,
            DevsoulInterface: IsDevsoulInterface<LinketImpl = LinketImpl>,
            F: Fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            B: IntoValue,
            Target: Send + FromValue,
            $($runtime_constant: Send + FromValue,)*
            $output: Send,
        {
            type FnOutput = $output;

            fn into_unveil_linket_impl(
                self,
                fn_wrapper: fn(
                    &[KiArgumentReprInterface],
                ) -> StandardKiControlFlow,
                fn_pointer: fn(Target, ($($runtime_constant,)*)) -> std::ops::ControlFlow<B, $output>,
            ) -> LinketImpl {
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
            ) -> StandardKiControlFlow<Self::FnOutput> {
                let ctx = DevsoulInterface::dev_eval_context();
                debug_assert_eq!(arguments.len(), 2);
                let KiArgumentReprInterface::Simple(target) = arguments[0] else {
                    unreachable!("expect ordinary argument")
                };
                let KiArgumentReprInterface::RuntimeConstants(
                    ref runtime_constants
                ) = arguments[1] else {
                    unreachable!("expect runtime constants, but got {:?} instead", arguments[1])
                };
                let slush_values = &mut SlushValues::default();
                let mut runtime_constants = runtime_constants.iter();
                ki_catch_unwind2!(
                    self.1,
                    |cf| match cf {
                        std::ops::ControlFlow::Continue(c) => KiControlFlow::Continue(c),
                        std::ops::ControlFlow::Break(b) => KiControlFlow::Return(b.into_value()),
                    },
                    <Target as FromValue>::from_value_temp(
                        ctx.eval_ki_repr_interface(target)?,
                        slush_values
                    ),
                    ($(<$runtime_constant as FromValue>::from_value_temp(
                        ctx.eval_val_runtime_constant(
                            *runtime_constants.next().expect("missing runtime constant")
                        ),
                        slush_values
                    ),)*)
                )
            }
        }
    };
}
