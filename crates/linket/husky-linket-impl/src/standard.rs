pub mod r#enum;
pub mod static_var;
pub mod r#struct;
pub mod ugly;
pub mod val;

pub use husky_standard_value::{
    frozen::ValueStands, value_conversion, DeprecatedValueLeashTest, FromValue, IntoValue, Value,
};

use self::StandardLinketImpl as LinketImpl;
#[cfg(test)]
use self::StandardLinketImpl as __LinketImpl;
use super::*;
use husky_decl_macro_utils::for_all_ritchie_tys;
use husky_devsoul_interface::{
    ki_repr::KiDomainReprInterface,
    pedestal::{IsPedestal, IsPedestalFull},
    VmArgumentValue,
};
use husky_value_protocol::presentation::EnumUnitValuePresenter;

// ad hoc
pub type Error = ();

pub type StandardLinketImplKiControlFlow<C = Value, B = Value> =
    husky_devsoul_interface::ki_control_flow::KiControlFlow<C, B, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardLinketImpl<Pedestal>
where
    Pedestal: IsPedestalFull,
{
    RitchieFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinketImplKiControlFlow,
        // todo: fn_vm_wrapper
        fn_pointer: fn(),
    },
    RitchieUnveilFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinketImplKiControlFlow,
        fn_pointer: fn(),
    },
    RitchieGn {
        gn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinketImplKiControlFlow,
    },
    // todo: this should be merged into RichieFn?
    EnumVariantConstructor {
        enum_variant_constructor_ki_wrapper: fn(&[KiArgumentReprInterface]) -> Value,
        enum_variant_constructor_vm_wrapper: fn(Vec<Value>) -> Value,
    },
    EnumVariantDestructor {
        enum_variant_destructor_wrapper: fn(Value) -> Vec<Value>,
    },
    EnumVariantDiscriminator {
        enum_variant_discriminator_wrapper: fn(Value) -> bool,
    },
    EnumVariantField {
        enum_variant_field_wrapper: fn(Value) -> Value,
    },
    /// used to get the json value of an enum u8-represented given only the index
    EnumUnitValuePresenter { presenter: EnumUnitValuePresenter },
    StructDestructor {
        struct_destructor_wrapper: fn(Value) -> Vec<Value>,
    },
    StructField {
        struct_field_wrapper: fn(Value) -> Value,
    },
    Val {
        init_item_path_id_interface: fn(ItemPathIdInterface),
        ki_wrapper: fn() -> StandardLinketImplKiControlFlow,
    },
    StaticVar {
        init_item_path_id_interface: fn(ItemPathIdInterface),
        set_up_for_testing: fn(usize),
        get_id: fn() -> Pedestal::StaticVarId,
        set_id: fn(Pedestal::StaticVarId),
    },
    // todo: memo
}

impl<Pedestal> Copy for StandardLinketImpl<Pedestal> where Pedestal: IsPedestalFull {}

impl<Pedestal> IsLinketImpl for StandardLinketImpl<Pedestal>
where
    Pedestal: IsPedestalFull,
{
    type Pedestal = Pedestal;
    type Value = Value;
    type Exception = Error;

    fn eval_ki(
        self,
        ki_repr: KiReprInterface,
        ki_argument_reprs: &[KiArgumentReprInterface],
        ctx: DevEvalContext<StandardLinketImpl<Pedestal>>,
    ) -> StandardLinketImplKiControlFlow {
        match self {
            StandardLinketImpl::RitchieFn { fn_ki_wrapper, .. } => fn_ki_wrapper(ki_argument_reprs),
            StandardLinketImpl::RitchieUnveilFn { fn_wrapper, .. } => fn_wrapper(ki_argument_reprs),
            StandardLinketImpl::RitchieGn { gn_ki_wrapper } => {
                todo!()
                // let value_at_generic_pedestal = ctx
                //     .eval_ki_repr_interface_at_generic_pedestal_with(
                //         ki_repr,
                //         generic_pedestal,
                //         gn_generic_wrapper,
                //         ki_argument_reprs,
                //     )?;
                // gn_specific_wrapper(ki_argument_reprs, value_at_generic_pedestal)
            }
            StandardLinketImpl::EnumVariantConstructor { .. } => todo!(),
            StandardLinketImpl::EnumVariantDestructor { .. } => todo!(),
            StandardLinketImpl::EnumVariantDiscriminator { .. } => todo!(),
            StandardLinketImpl::EnumVariantField { .. } => todo!(),
            StandardLinketImpl::EnumUnitValuePresenter { .. } => {
                unreachable!("this linket is not meant to be evaluated like this")
            }
            StandardLinketImpl::StructDestructor {
                struct_destructor_wrapper,
            } => todo!(),
            StandardLinketImpl::StructField {
                struct_field_wrapper,
            } => {
                debug_assert_eq!(ki_argument_reprs.len(), 1);
                let KiArgumentReprInterface::Simple(owner) = ki_argument_reprs[0] else {
                    unreachable!()
                };
                let owner = ctx.eval_ki_repr_interface(owner)?;
                StandardLinketImplKiControlFlow::Continue(struct_field_wrapper(owner))
            }
            StandardLinketImpl::StaticVar { .. } => todo!(),
            StandardLinketImpl::Val {
                ki_wrapper,
                init_item_path_id_interface: set_item_path_id_interface,
            } => {
                debug_assert!(ki_argument_reprs.is_empty());
                ki_wrapper()
            }
        }
    }

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> husky_devsoul_interface::vm_control_flow::LinketImplVmControlFlow<Self> {
        todo!()
    }

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter {
        match self {
            StandardLinketImpl::EnumUnitValuePresenter { presenter } => presenter,
            _ => unreachable!(),
        }
    }

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId {
        let StandardLinketImpl::StaticVar { get_id, .. } = self else {
            unreachable!()
        };
        get_id()
    }

    fn init_item_path_id_interface(self, item_path_id_interface: ItemPathIdInterface) {
        match self {
            StandardLinketImpl::RitchieFn { .. } => (),
            StandardLinketImpl::RitchieUnveilFn { .. } => (),
            StandardLinketImpl::RitchieGn { .. } => (),
            StandardLinketImpl::EnumVariantConstructor { .. } => (),
            StandardLinketImpl::EnumVariantDestructor { .. } => (),
            StandardLinketImpl::EnumVariantDiscriminator { .. } => (),
            StandardLinketImpl::EnumVariantField { .. } => (),
            StandardLinketImpl::EnumUnitValuePresenter { .. } => (),
            StandardLinketImpl::StructDestructor { .. } => (),
            StandardLinketImpl::StructField { .. } => (),
            StandardLinketImpl::Val {
                init_item_path_id_interface,
                ..
            }
            | StandardLinketImpl::StaticVar {
                init_item_path_id_interface,
                ..
            } => init_item_path_id_interface(item_path_id_interface),
        }
    }
}

pub struct FnLinketImplSource<Pedestal, DevsoulInterface, T>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub T,
);

for_all_ritchie_tys! {impl_is_fn_linket_impl_source}

pub struct UnveilFnLinketImplSource<Pedestal, DevsoulInterface, T>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub T,
);

for_all_ritchie_tys! {impl_is_unveil_fn_linket_impl_source}

pub trait IsGnItem {
    type LinketImpl: IsLinketImpl;

    fn generic_pedestal(
        specific_pedestal: <Self::LinketImpl as IsLinketImpl>::Pedestal,
    ) -> <Self::LinketImpl as IsLinketImpl>::Pedestal;

    type ValueAtGenericPedestal;

    /// compute `generic_pedestal` here for efficiency
    fn train(
        ki_domain_repr: KiDomainReprInterface,
        val_argument_reprs: &[KiArgumentReprInterface],
    ) -> LinketImplKiControlFlow<Self::LinketImpl, Self::ValueAtGenericPedestal>;

    type EvalOutput;

    fn eval(
        val_argument_reprs: &[KiArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> Self::EvalOutput;
}

#[macro_export]
macro_rules! gn_linket_impl {
    ($gn_item: ty) => {{
        __LinketImpl::RitchieGn {
            gn_ki_wrapper: <$gn_item>::gn_ki_wrapper,
        }
    }};
}

#[test]
#[ignore]
fn gn_linket_impl_works() {
    todo!()
}

// fn gn_ki_wrapper(val_argument_reprs: &[__KiArgumentReprInterface]) -> __KiControlFlow {
//     let value_stands = &mut Default::default();
//     let value_at_generic_pedestal: &<$gn_item as __IsGnItem>::ValueAtGenericPedestal =
//         <&<$gn_item as __IsGnItem>::ValueAtGenericPedestal as FromValue>::from_value_temp(
//             value_at_generic_pedestal,
//             value_stands,
//         );
//     // todo: catch unwind
//     __KiControlFlow::Continue(
//         __ValueLeashTest(<$gn_item as __IsGnItem>::eval(
//             val_argument_reprs,
//             value_at_generic_pedestal,
//         ))
//         .into_value(),
//     )
// }

#[macro_export]
macro_rules! class_specific_leashed_field_into_value {
    (copyable $expr: expr) => {{
        (*$expr).into_value()
    }};
    (vec $expr: expr) => {{
        Leash::<[_]>::new($expr).into_value()
    }};
    (other $expr: expr) => {{
        Leash::new($expr).into_value()
    }};
}
