use super::*;
use husky_devsoul_interface::{
    pedestal::IsPedestal, vm_control_flow::LinkageImplVmControlFlow, IsLinkageImpl, VmArgumentValue,
};
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use husky_virtual_value::value::Value;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VirtualLinkageImpl(Linkage);

impl std::ops::Deref for VirtualLinkageImpl {
    type Target = Linkage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Linkage> for VirtualLinkageImpl {
    fn from(linkage: Linkage) -> Self {
        Self(linkage)
    }
}

impl IsLinkageImpl for VirtualLinkageImpl {
    type Pedestal = ();

    type Value = Value;

    // ad hoc
    type Exception = ();

    fn eval_ki(
        self,
        ki_repr_interface: husky_devsoul_interface::ugly::__KiReprInterface,
        ctx: husky_devsoul_interface::DevEvalContext<Self>,
        arguments: &[husky_devsoul_interface::ugly::__KiArgumentReprInterface],
    ) -> husky_devsoul_interface::LinkageImplKiControlFlow<Self> {
        todo!()
    }

    fn eval_vm(
        self,
        mut arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinkageImplVmControlFlow<Self> {
        use husky_devsoul_interface::vm_control_flow::VmControlFlow::*;

        let db: &::salsa::Db = db.downcast_ref().unwrap();
        match self.data(db) {
            LinkageData::MajorFunctionRitchie {
                path,
                instantiation,
            } => todo!(),
            LinkageData::MajorStaticVar {
                path,
                instantiation,
            } => todo!(),
            LinkageData::MajorVal {
                path,
                instantiation,
            } => todo!(),
            LinkageData::MemoizedField {
                path,
                instantiation,
            } => todo!(),
            LinkageData::MethodRitchie {
                path,
                instantiation,
            } => todo!(),
            LinkageData::AssocRitchie {
                path,
                instantiation,
            } => todo!(),
            LinkageData::UnveilAssocRitchie {
                path,
                instantiation,
            } => todo!(),
            LinkageData::StructConstructor {
                path,
                instantiation,
            } => todo!(),
            LinkageData::StructDestructor { self_ty } => todo!(),
            LinkageData::EnumVariantConstructor {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinkageData::EnumVariantDiscriminator {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinkageData::EnumVariantDestructor {
                self_ty,
                path,
                instantiation,
            } => todo!(),
            LinkageData::StructField { self_ty, field } => todo!(),
            LinkageData::EnumVariantField {
                path,
                instantiation,
                field,
            } => todo!(),
            LinkageData::Index => todo!(),
            LinkageData::VecConstructor { element_ty } => {
                let VmArgumentValue::Variadic(elements) = arguments.pop().unwrap() else {
                    unreachable!()
                };
                Continue(Value::Vec(elements))
            }
            LinkageData::TypeDefault { ty } => todo!(),
            LinkageData::EnumUnitToJsonValue { ty_path } => todo!(),
        }
    }

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter {
        todo!()
    }

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId {
        todo!()
    }
}
