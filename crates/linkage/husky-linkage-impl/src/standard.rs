pub mod r#enum;
pub mod static_var;
pub mod r#struct;
pub mod ugly;

pub use husky_standard_value::{
    frozen::ValueStands, value_conversion, FromValue, IntoValue, Value, ValueLeashTest,
};

use self::StandardLinkageImpl as LinkageImpl;
#[cfg(test)]
use self::StandardLinkageImpl as __LinkageImpl;
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

pub type StandardLinkageImplKiControlFlow<C = Value, B = Value> =
    husky_devsoul_interface::ki_control_flow::KiControlFlow<C, B, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardLinkageImpl<Pedestal>
where
    Pedestal: IsPedestalFull,
{
    RitchieFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_ki_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinkageImplKiControlFlow,
        // todo: fn_vm_wrapper
        fn_pointer: fn(),
    },
    RitchieUnveilFn {
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(&[KiArgumentReprInterface]) -> StandardLinkageImplKiControlFlow,
        fn_pointer: fn(),
    },
    RitchieGn {
        gn_ki_wrapper: (),
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
    EnumUnitValuePresenter {
        presenter: EnumUnitValuePresenter,
    },
    StructDestructor {
        struct_destructor_wrapper: fn(Value) -> Vec<Value>,
    },
    StructField {
        struct_field_wrapper: fn(Value) -> Value,
    },
    StaticVar {
        set_up_for_testing: fn(usize),
        get_id: fn() -> Pedestal::StaticVarId,
        set_id: fn(Pedestal::StaticVarId),
    },
}

impl<Pedestal> Copy for StandardLinkageImpl<Pedestal> where Pedestal: IsPedestalFull {}

impl<Pedestal> IsLinkageImpl for StandardLinkageImpl<Pedestal>
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
        ctx: DevEvalContext<StandardLinkageImpl<Pedestal>>,
    ) -> StandardLinkageImplKiControlFlow {
        match self {
            StandardLinkageImpl::RitchieFn { fn_ki_wrapper, .. } => {
                fn_ki_wrapper(ki_argument_reprs)
            }
            StandardLinkageImpl::RitchieUnveilFn { fn_wrapper, .. } => {
                fn_wrapper(ki_argument_reprs)
            }
            StandardLinkageImpl::RitchieGn { gn_ki_wrapper } => {
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
            StandardLinkageImpl::EnumVariantConstructor { .. } => todo!(),
            StandardLinkageImpl::EnumVariantDestructor { .. } => todo!(),
            StandardLinkageImpl::EnumVariantDiscriminator { .. } => todo!(),
            StandardLinkageImpl::EnumVariantField { .. } => todo!(),
            StandardLinkageImpl::EnumUnitValuePresenter { .. } => {
                unreachable!("this linkage is not meant to be evaluated like this")
            }
            StandardLinkageImpl::StructDestructor {
                struct_destructor_wrapper,
            } => todo!(),
            StandardLinkageImpl::StructField {
                struct_field_wrapper,
            } => {
                debug_assert_eq!(ki_argument_reprs.len(), 1);
                let KiArgumentReprInterface::Simple(owner) = ki_argument_reprs[0] else {
                    unreachable!()
                };
                let owner = ctx.eval_ki_repr_interface(owner)?;
                StandardLinkageImplKiControlFlow::Continue(struct_field_wrapper(owner))
            }
            StandardLinkageImpl::StaticVar {
                set_up_for_testing,
                get_id,
                set_id,
            } => todo!(),
        }
    }

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> husky_devsoul_interface::vm_control_flow::LinkageImplVmControlFlow<Self> {
        todo!()
    }

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter {
        match self {
            StandardLinkageImpl::EnumUnitValuePresenter { presenter } => presenter,
            _ => unreachable!(),
        }
    }

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId {
        let StandardLinkageImpl::StaticVar {
            set_up_for_testing,
            get_id,
            set_id,
        } = self
        else {
            unreachable!()
        };
        get_id()
    }
}

pub struct FnLinkageImplSource<Pedestal, DevsoulInterface, T>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub T,
);

for_all_ritchie_tys! {impl_is_fn_linkage_impl_source}

pub struct UnveilFnLinkageImplSource<Pedestal, DevsoulInterface, T>(
    pub std::marker::PhantomData<(Pedestal, DevsoulInterface)>,
    pub T,
);

for_all_ritchie_tys! {impl_is_unveil_fn_linkage_impl_source}

pub trait IsGnItem {
    type LinkageImpl: IsLinkageImpl;

    fn generic_pedestal(
        specific_pedestal: <Self::LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> <Self::LinkageImpl as IsLinkageImpl>::Pedestal;

    type ValueAtGenericPedestal;

    /// compute `generic_pedestal` here for efficiency
    fn train(
        ki_domain_repr: KiDomainReprInterface,
        val_argument_reprs: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<Self::LinkageImpl, Self::ValueAtGenericPedestal>;

    type EvalOutput;

    fn eval(
        val_argument_reprs: &[KiArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> Self::EvalOutput;
}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: ty) => {{
        fn gn_ki_wrapper(val_argument_reprs: &[__KiArgumentReprInterface]) -> __KiControlFlow {
            let value_stands = &mut Default::default();
            let value_at_generic_pedestal: &<$gn_item as __IsGnItem>::ValueAtGenericPedestal =
                <&<$gn_item as __IsGnItem>::ValueAtGenericPedestal as FromValue>::from_value_temp(
                    value_at_generic_pedestal,
                    value_stands,
                );
            // todo: catch unwind
            __KiControlFlow::Continue(
                __ValueLeashTest(<$gn_item as __IsGnItem>::eval(
                    val_argument_reprs,
                    value_at_generic_pedestal,
                ))
                .into_value(),
            )
        }
        __LinkageImpl::RitchieGn { gn_ki_wrapper }
    }};
}

#[macro_export]
macro_rules! enum_index_presenter_linkage_impl {
    ($ty: ty) => {
        __LinkageImpl::EnumUnitValuePresenter {
            presenter: |index: usize, _, _| {
                let index: u8 = index.try_into().unwrap();
                let slf: $ty = unsafe { std::mem::transmute(index) };
                __ValuePresentation::AdHoc(format!("{slf:?}"))
            },
        }
    };
}
