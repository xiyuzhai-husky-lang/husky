use super::*;
use husky_devsoul_interface::{
    pedestal::IsPedestal, vm_control_flow::LinketImplVmControlFlow, IsLinketImpl, VmArgumentValue,
};
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use husky_virtual_value::value::Value;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VirtualLinketImpl(Linket);

impl std::ops::Deref for VirtualLinketImpl {
    type Target = Linket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Linket> for VirtualLinketImpl {
    fn from(linket: Linket) -> Self {
        Self(linket)
    }
}

impl IsLinketImpl for VirtualLinketImpl {
    type Pedestal = ();

    type Value = Value;

    // ad hoc
    type Exception = ();

    fn eval_ki(
        self,
        ki_repr_interface: husky_devsoul_interface::ugly::__KiReprInterface,
        arguments: &[husky_devsoul_interface::ugly::__KiArgumentReprInterface],
        ctx: husky_devsoul_interface::DevEvalContext<Self>,
    ) -> husky_devsoul_interface::LinketImplKiControlFlow<Self> {
        todo!()
    }

    fn eval_vm(
        self,
        mut arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinketImplVmControlFlow<Self> {
        use husky_devsoul_interface::vm_control_flow::VmControlFlow::*;

        let db: &::salsa::Db = db.downcast_ref().unwrap();
        match self.data(db) {
            LinketData::MajorFunctionRitchie {
                path,
                instantiation,
            } => todo!(),
            LinketData::MajorStaticVar {
                path,
                instantiation,
            } => todo!(),
            LinketData::MajorVal {
                path,
                instantiation,
            } => todo!(),
            LinketData::MemoizedField {
                path,
                instantiation,
            } => todo!(),
            LinketData::MethodRitchie {
                path,
                instantiation,
            } => todo!(),
            LinketData::AssocRitchie {
                path,
                instantiation,
            } => todo!(),
            LinketData::UnveilAssocRitchie {
                path,
                instantiation,
            } => todo!(),
            LinketData::StructConstructor {
                path,
                instantiation,
            } => todo!(),
            LinketData::StructDestructor { self_ty } => todo!(),
            LinketData::EnumVariantConstructor {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinketData::EnumVariantDiscriminator {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinketData::EnumVariantDestructor {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinketData::StructField { self_ty, field } => todo!(),
            LinketData::EnumVariantField {
                path,
                instantiation,
                field,
            } => todo!(),
            LinketData::Index => todo!(),
            LinketData::VecConstructor { element_ty } => {
                let VmArgumentValue::Variadic(elements) = arguments.pop().unwrap() else {
                    unreachable!()
                };
                Continue(Value::Vec(elements))
            }
            LinketData::TypeDefault { ty } => todo!(),
            LinketData::EnumUnitToJsonValue { ty_path } => todo!(),
        }
    }

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter {
        todo!()
    }

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId {
        todo!()
    }
}
