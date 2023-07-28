use husky_vm::GenericArgument;

use crate::__rust_code_gen__::__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE;

use super::*;

static_mod! { normalize = { normalize_vmax_f32 } }

pub static NORMALIZE_VMAX_F32_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "normalize_vmax_f32",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[
            StaticParameter {
                name: "f",
                modifier: ParameterModifier::None,
                ty: "f32",
            },
            StaticParameter {
                name: "label0",
                modifier: ParameterModifier::None,
                ty: "TargetOutput",
            },
        ],
        variadic_template: StaticVariadicParameterDecl::None,
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        linkage: __LinkageGroup::Model(__ModelLinkageGroup(&NormalizeVmaxF32)),
    },
    dev_src: static_dev_src!(),
};

#[derive(Debug)]
struct NormalizeVmaxF32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NormalizeVmaxF32Internal {
    vmax: ordered_float::NotNan<f32>,
}

impl __StaticInfo for NormalizeVmaxF32Internal {
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

impl<'eval> __Registrable<'eval> for NormalizeVmaxF32Internal {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE)
    }
}

impl Model for NormalizeVmaxF32 {
    type Internal = NormalizeVmaxF32Internal; // most likely labels

    fn train<'eval>(
        &self,
        opds: Vec<GenericArgument>,
        labels: Vec<i32>,
    ) -> __VMResult<Self::Internal> {
        let label0: &__VirtualEnum = opds[1].value().downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
        let label0 = label0.kind_idx;
        let mut values: Vec<ordered_float::NotNan<f32>> = vec![];
        for (value, label) in std::iter::zip(opds[0].values().iter(), labels.iter()) {
            if label0 == *label {
                let value = value.downcast_f32();
                assert!(value >= 0.0);
                values.push(ordered_float::NotNan::new(value).unwrap())
            }
        }
        let vmax = *values.iter().max().unwrap();
        Ok(NormalizeVmaxF32Internal { vmax })
    }

    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register<'eval>],
    ) -> __VMResult<__Register<'eval>> {
        let vmax = internal.vmax.into_inner();
        Ok(if vmax > 0.0 {
            arguments[0].downcast_f32() / vmax
        } else {
            arguments[0].downcast_f32()
        }
        .to_register())
    }

    fn internal_ty_vtable() -> &'static husky_vm::__RegisterTyVTable {
        &__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE
    }
}
