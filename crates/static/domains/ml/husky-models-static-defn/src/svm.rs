use crate::__rust_code_gen__::__SVM_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE;

use super::*;

static_mod! { svm = { svm_with_vmax_normalized } }

pub static SVM_WITH_VMAX_NORMALIZED_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "svm_with_vmax_normalized",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[StaticParameter {
            name: "label0",
            modifier: ParameterModifier::None,
            ty: "TargetOutput",
        }],
        variadic_template: StaticVariadicTemplate::SingleTyped { ty: "f32" },
        output_ty: "f32",
        output_liason: OutputModifier::Transfer,
        linkage: __Linkage::Model(__ModelLinkage(&SvmWithVmaxNormalized)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct SvmWithVmaxNormalized;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SvmWithVmaxNormalizedInternal {
    vmax: ordered_float::NotNan<f32>,
}

impl __StaticInfo for SvmWithVmaxNormalizedInternal {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for SvmWithVmaxNormalizedInternal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__SVM_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE)
    }
}

impl Model for SvmWithVmaxNormalized {
    type Internal = SvmWithVmaxNormalizedInternal;

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        todo!()
    }

    fn train<'eval>(
        &self,
        opds: Vec<(Vec<__Register<'eval>>, __Register<'eval>)>,
    ) -> __VMResult<Self::Internal> {
        todo!()
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        todo!()
    }
}
